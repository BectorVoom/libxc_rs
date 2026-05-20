//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2805/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2805<F: Float>(t23384: F, t689: F, t779: F, t14987: F, t18797: F, t23388: F, t786: F, t789: F, t15011: F, t39549: F, t50155: F, t50166: F, t50178: F, t6049: F, t6072: F, t61324: F, t61330: F, t61337: F, t61344: F, t61348: F, t61351: F) -> F {
    let t75950 = t689 * t779 * t23384;
    let t75956 = t14987 * t18797;
    let t75961 = t786 * t23388 * t789;
    let t75970 = F::cast_from(0.54878743191129263322e-2_f64) * t75950 - F::cast_from(0.33133632253434461091e-3_f64) * t50155 - F::cast_from(0.19514881078765566037e-2_f64) * t61324 - F::cast_from(0.39029762157531132076e-1_f64) * t61330 - F::cast_from(0.51220160311720645768e-1_f64) * t50166 - F::cast_from(0.29272321618148349057e-1_f64) * t75956 - F::cast_from(0.58911598146606471821e-3_f64) * t50178 - F::cast_from(0.43902994552903410656e-1_f64) * t61337 + F::cast_from(0.9757440539382783019e-2_f64) * t75961 + F::cast_from(0.32927245914677557992e-1_f64) * t61344 + F::cast_from(0.39512695097613069591e1_f64) * t15011 * t6049 - F::cast_from(0.17563392970889009434e0_f64) * t61348 - F::cast_from(0.19756347548806534796e1_f64) * t15011 * t6072 - t39549 + F::cast_from(0.29272321618148349057e-1_f64) * t61351;
    t75970
}
