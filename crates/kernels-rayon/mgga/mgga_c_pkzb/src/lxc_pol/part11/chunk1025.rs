//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1025/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1025(t11327: f64, t898: f64, t398: f64, t19: f64, t297: f64, t326: f64, t397: f64, t10115: f64, t1167: f64, t2888: f64, t1227: f64, t3874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11329 = 0.10389515463408878255e3_f64 * t898 * t11327;
    let t11333 = t398 * t398;
    let t11335 = 1.0_f64 / t19 / t11333;
    let t11338 = t397 * t326 * t11335 * t297;
    let t11341 = t10115 * t1167;
    let t11342 = t2888 * t11341;
    let t11345 = t3874 * t1227;
    (t11329, t11335, t11338, t11341, t11342, t11345)
}
