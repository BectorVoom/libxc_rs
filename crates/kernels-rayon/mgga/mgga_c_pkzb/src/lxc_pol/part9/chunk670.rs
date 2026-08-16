//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 670/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk670(t3073: f64, t851: f64, t2240: f64, t2175: f64, t2246: f64, t3017: f64, t3028: f64, t1189: f64, t862: f64, t1197: f64, t870: f64, t2224: f64, t2264: f64, t2269: f64, t3042: f64, t3047: f64, t3053: f64, t3055: f64, t3059: f64, t3063: f64, t3067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3074 = t3073 * t851;
    let t3076 = 0.16081979498692535067e2_f64 * t2240 * t3074;
    let t3080 = t2246 - 0.17123333333333333333e-1_f64 * t2175 - 0.17123333333333333333e-1_f64 * t3017 + 0.5137e-1_f64 * t3028;
    let t3083 = t1189 * t862;
    let t3088 = t1197 * t870;
    let t3102 = -0.17648625e1_f64 * t3042 + 0.3529725e1_f64 * t3047 + t2264 - 0.516475e0_f64 * t2175 - 0.516475e0_f64 * t3017 + 0.1549425e1_f64 * t3028 + 0.31558125e0_f64 * t3053 + 0.6311625e0_f64 * t3055 + t2269 - 0.20839e0_f64 * t2224 - 0.20839e0_f64 * t3059 + 0.312585e0_f64 * t3063 + 0.312585e0_f64 * t3067;
    (t3074, t3076, t3080, t3083, t3088, t3102)
}
