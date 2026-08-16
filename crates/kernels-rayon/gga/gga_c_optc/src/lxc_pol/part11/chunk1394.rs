//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1394/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1394(t415: f64, t58848: f64, t58860: f64, t241: f64, t1239: f64, t16241: f64) -> (f64, f64, f64) {
    let t58862 = (t58848 + t58860) * t415;
    let t58864 = 0.19751789702565206229e-1_f64 * t241 * t58862;
    let t58865 = t16241 * t1239;
    (t58862, t58864, t58865)
}
