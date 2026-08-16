//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 421/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk421(t48: f64, t1403: f64, t1407: f64, t476: f64, t53: f64, t1413: f64, t1416: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1523 = 1.0_f64 / t48;
    let t1524 = t1523 * t1403;
    let t1526 = t476 * t1407;
    let t1528 = 1.0_f64 / t53;
    let t1529 = t1528 * t1413;
    let t1531 = t478 * t1416;
    let t1533 = -t1524 / 9.0_f64 + t1526 / 3.0_f64 - t1529 / 9.0_f64 + t1531 / 3.0_f64;
    (t1523, t1524, t1526, t1528, t1529, t1531, t1533)
}
