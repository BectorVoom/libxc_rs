#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 3.

pub mod gga_c_bmk;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_ft97;
// pub mod gga_c_sg4;
// pub mod gga_x_lcgau;
