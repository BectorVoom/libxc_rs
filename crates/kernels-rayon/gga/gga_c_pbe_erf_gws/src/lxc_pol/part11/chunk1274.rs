//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1274/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1274(t46598: f64, t13440: f64, t3781: f64, t850: f64, t860: f64, t1134: f64, t49841: f64, t1123: f64, t12381: f64, t339: f64, t46615: f64, t2080: f64, t2083: f64, t2085: f64, t48997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50349 = 7.0_f64 / 6.0_f64 * t46598;
    let t50353 = t850 * t3781 * t13440 * t860 / 32.0_f64;
    let t50354 = t1134 * t49841;
    let t50362 = t850 * t1123 * t12381 * t339 * t860 / 96.0_f64;
    let t50363 = 7.0_f64 / 36.0_f64 * t46615;
    let t50368 = t2080 * t48997 * t2083 * t2085 * t860 / 32.0_f64;
    (t50349, t50353, t50354, t50362, t50363, t50368)
}
