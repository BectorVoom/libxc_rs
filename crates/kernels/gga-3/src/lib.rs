#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 3.

pub mod gga_c_lyp;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_acggap;
// pub mod gga_c_pbe_erf_gws;
// pub mod gga_c_q2d;
// pub mod gga_x_hjs;
