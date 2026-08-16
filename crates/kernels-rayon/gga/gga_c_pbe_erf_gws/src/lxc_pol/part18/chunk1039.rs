//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1039/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1039(t11514: f64, t3140: f64, t3235: f64, t2170: f64, t3814: f64, t8840: f64, t2168: f64, t11478: f64, t6287: f64, t3138: f64, t8884: f64, t8890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11640 = t3235 * t11514 * t3140;
    let t11644 = t2170 * t8840 * t3814;
    let t11646 = t2168 * t11644 / 24.0_f64;
    let t11648 = t2170 * t11478 * t6287;
    let t11650 = t3138 * t11648 / 24.0_f64;
    let t11651 = t8884 * t8890;
    (t11640, t11644, t11646, t11648, t11650, t11651)
}
