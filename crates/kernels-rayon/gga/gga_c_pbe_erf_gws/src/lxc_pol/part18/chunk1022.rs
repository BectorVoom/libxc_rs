//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1022/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1022(t11438: f64, t9499: f64, t1134: f64, t820: f64, t2306: f64, t9386: f64, t3123: f64, t8824: f64, t11416: f64, t11418: f64, t11421: f64, t11423: f64, t11427: f64, t11431: f64, t11435: f64, t6275: f64, t6637: f64, t8823: f64, t9342: f64, t9637: f64) -> (f64, f64, f64, f64, f64) {
    let t11439 = t9499 * t11438;
    let t11442 = t1134 * t820;
    let t11443 = t2306 * t11442;
    let t11444 = t9386 * t11443;
    let t11447 = t3123 * t8824;
    let t11448 = 7.0_f64 / 144.0_f64 * t11447;
    let t11449 = t11416 + t11418 + t11421 + t6637 * t11423 / 384.0_f64 + t6275 * t11427 / 96.0_f64 + t6275 * t11431 / 96.0_f64 + t6275 * t11435 / 96.0_f64 + t9637 * t11439 / 128.0_f64 - t6637 * t11444 / 192.0_f64 + t11448 + t8823 + t9342;
    (t11439, t11443, t11444, t11448, t11449)
}
