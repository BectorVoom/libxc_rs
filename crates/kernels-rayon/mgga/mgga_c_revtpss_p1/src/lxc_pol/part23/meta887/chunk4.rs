//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2805/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2805(t23384: f64, t689: f64, t779: f64, t14987: f64, t18797: f64, t23388: f64, t786: f64, t789: f64, t15011: f64, t39549: f64, t50155: f64, t50166: f64, t50178: f64, t6049: f64, t6072: f64, t61324: f64, t61330: f64, t61337: f64, t61344: f64, t61348: f64, t61351: f64) -> f64 {
    let t75950 = t689 * t779 * t23384;
    let t75956 = t14987 * t18797;
    let t75961 = t786 * t23388 * t789;
    let t75970 = 0.54878743191129263322e-2_f64 * t75950 - 0.33133632253434461091e-3_f64 * t50155 - 0.19514881078765566037e-2_f64 * t61324 - 0.39029762157531132076e-1_f64 * t61330 - 0.51220160311720645768e-1_f64 * t50166 - 0.29272321618148349057e-1_f64 * t75956 - 0.58911598146606471821e-3_f64 * t50178 - 0.43902994552903410656e-1_f64 * t61337 + 0.9757440539382783019e-2_f64 * t75961 + 0.32927245914677557992e-1_f64 * t61344 + 0.39512695097613069591e1_f64 * t15011 * t6049 - 0.17563392970889009434e0_f64 * t61348 - 0.19756347548806534796e1_f64 * t15011 * t6072 - t39549 + 0.29272321618148349057e-1_f64 * t61351;
    t75970
}
