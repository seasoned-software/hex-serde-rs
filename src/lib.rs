//! Integration between hex and serde
//!
//! # Examples
//!
//! ```
//! # extern crate hex_serde;
//! # #[macro_use]
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! #[derive(Serialize, Deserialize)]
//! struct Sha256(#[serde(with = "hex_serde")] [u8; 32]);
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "hex_serde")] icecream: Vec<u8>,
//!     count: u64,
//! }
//! # fn main() {}
//! ```

extern crate hex;
extern crate serde;

use hex::{FromHex, FromHexError, ToHex};
use serde::de::Visitor;
use serde::{de, Deserializer, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// A serializer that first encodes the argument as a hex-string
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let output: String = value.encode_hex();
    serializer.serialize_str(&output)
}

/// A deserializer that first encodes the argument as a hex-string
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromHex<Error = FromHexError>,
{
    struct HexVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for HexVisitor<T>
    where
        T: FromHex<Error = FromHexError>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "hex ASCII text")
        }

        fn visit_str<E>(self, v: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            T::from_hex(v).map_err(|e| match e {
                FromHexError::InvalidHexCharacter { c, index } => E::invalid_value(
                    de::Unexpected::Char(c),
                    &format!("Unexpected character {:?} as position {}", c, index).as_str(),
                ),
                FromHexError::InvalidStringLength => {
                    E::invalid_length(v.len(), &"Unexpected length of hex string")
                }
                FromHexError::OddLength => E::invalid_length(v.len(), &"Odd length of hex string"),
            })
        }
    }

    deserializer.deserialize_str(HexVisitor(PhantomData))
}
