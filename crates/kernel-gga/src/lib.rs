#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations from maple2c.
//!
//! 131 GGA functionals total across 5 sub-crates.
//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under
//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.

// Re-export sub-crates containing compiled GGA functionals.
pub use libxc_kernel_gga_1 as batch1;
pub use libxc_kernel_gga_2 as batch2;
pub use libxc_kernel_gga_3 as batch3;
pub use libxc_kernel_gga_4 as batch4;
pub use libxc_kernel_gga_5 as batch5;
