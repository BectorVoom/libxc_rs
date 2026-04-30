#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations from maple2c.
//!
//! 131 GGA functionals total across 22 sub-crates.
//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under
//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.

// Re-export sub-crates containing compiled GGA functionals.
pub use libxc_kernel_gga_1a as batch1a;
pub use libxc_kernel_gga_1b as batch1b;
pub use libxc_kernel_gga_1c as batch1c;
pub use libxc_kernel_gga_1d as batch1d;
pub use libxc_kernel_gga_2a as batch2a;
pub use libxc_kernel_gga_2b as batch2b;
pub use libxc_kernel_gga_2c as batch2c;
pub use libxc_kernel_gga_2d as batch2d;
pub use libxc_kernel_gga_3a as batch3a;
pub use libxc_kernel_gga_3b as batch3b;
pub use libxc_kernel_gga_3c as batch3c;
pub use libxc_kernel_gga_3d as batch3d;
pub use libxc_kernel_gga_3e as batch3e;
pub use libxc_kernel_gga_4a as batch4a;
pub use libxc_kernel_gga_4b as batch4b;
pub use libxc_kernel_gga_4c as batch4c;
pub use libxc_kernel_gga_4d as batch4d;
pub use libxc_kernel_gga_4e as batch4e;
pub use libxc_kernel_gga_4f as batch4f;
pub use libxc_kernel_gga_4g as batch4g;
pub use libxc_kernel_gga_5a as batch5a;
pub use libxc_kernel_gga_5b as batch5b;
pub use libxc_kernel_gga_5c as batch5c;
pub use libxc_kernel_gga_5d as batch5d;
pub use libxc_kernel_gga_5e as batch5e;
pub use libxc_kernel_gga_5f as batch5f;
pub use libxc_kernel_gga_5g as batch5g;
pub use libxc_kernel_gga_6a as batch6a;
pub use libxc_kernel_gga_6b as batch6b;
pub use libxc_kernel_gga_6c as batch6c;
pub use libxc_kernel_gga_6d as batch6d;
pub use libxc_kernel_gga_7a as batch7a;
pub use libxc_kernel_gga_7b as batch7b;
pub use libxc_kernel_gga_7c as batch7c;
pub use libxc_kernel_gga_7d as batch7d;
pub use libxc_kernel_gga_8a as batch8a;
pub use libxc_kernel_gga_8b as batch8b;
pub use libxc_kernel_gga_8c as batch8c;
pub use libxc_kernel_gga_8d as batch8d;
pub use libxc_kernel_gga_9a as batch9a;
pub use libxc_kernel_gga_9b as batch9b;
pub use libxc_kernel_gga_9c as batch9c;
pub use libxc_kernel_gga_10a as batch10a;
pub use libxc_kernel_gga_10b as batch10b;
pub use libxc_kernel_gga_10c as batch10c;
pub use libxc_kernel_gga_10d as batch10d;
pub use libxc_kernel_gga_11 as batch11;
pub use libxc_kernel_gga_12 as batch12;
pub use libxc_kernel_gga_13 as batch13;
pub use libxc_kernel_gga_14 as batch14;
pub use libxc_kernel_gga_15 as batch15;
pub use libxc_kernel_gga_16 as batch16;
pub use libxc_kernel_gga_17 as batch17;
pub use libxc_kernel_gga_18 as batch18;
pub use libxc_kernel_gga_19 as batch19;
pub use libxc_kernel_gga_20 as batch20;
pub use libxc_kernel_gga_21 as batch21;
pub use libxc_kernel_gga_22 as batch22;
