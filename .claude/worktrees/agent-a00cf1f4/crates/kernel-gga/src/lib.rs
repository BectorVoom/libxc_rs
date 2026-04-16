#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations from maple2c.
//!
//! 131 GGA functionals total across 22 sub-crates.
//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under
//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.

// Re-export sub-crates containing compiled GGA functionals.
pub use libxc_kernel_gga_1 as batch1;
pub use libxc_kernel_gga_2 as batch2;
pub use libxc_kernel_gga_3 as batch3;
pub use libxc_kernel_gga_4 as batch4;
pub use libxc_kernel_gga_5 as batch5;
pub use libxc_kernel_gga_6 as batch6;
pub use libxc_kernel_gga_7 as batch7;
pub use libxc_kernel_gga_8 as batch8;
pub use libxc_kernel_gga_9 as batch9;
pub use libxc_kernel_gga_10 as batch10;
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
