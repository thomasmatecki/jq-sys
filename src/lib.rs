#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;

unsafe impl Send for jq_state {}
unsafe impl Sync for jq_state {}

pub use bindings::*;
