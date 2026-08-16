//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 883/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk883(t1339: f64, t16451: f64, t1971: f64, t1331: f64, t8: f64, t147: f64, t551: f64, t553: f64, t6041: f64, t6047: f64, t1472: f64, t2003: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16454 = 0.29725654166942986832e-2_f64 * t1339 * t16451 * t1971;
    let t16463 = 1.0_f64 / t8 / t1331;
    let t16465 = t16463 * t147 * t551;
    let t16467 = 0.74395492895254307406e-5_f64 * t16465 * t553;
    let t16471 = 0.1035981803916141664e0_f64 * t6041 * t553;
    let t16480 = 0.12408369628826103546e0_f64 * t6047 * t553;
    let t16485 = 0.19878653761973934499e-1_f64 * t2003 * t1472 * t671;
    (t16454, t16463, t16465, t16467, t16471, t16480, t16485)
}
