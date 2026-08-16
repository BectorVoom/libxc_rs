//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 989/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk989(t109: f64, t138: f64, t3665: f64, t3670: f64, t1179: f64, t621: f64, t634: f64, t1036: f64, t3878: f64, t1040: f64, t1044: f64, t409: f64) -> (f64, f64, f64, f64) {
    let t8633 = 36.84616320282908_f64 * t138 * t109 * t3665 * t3670;
    let t8637 = 0.22161481481481482_f64 * t138 * t1179 * t621 * t634;
    let t8640 = 0.14246666666666666_f64 * t138 * t3878 * t1036;
    let t8644 = 2.2911460125803966_f64 * t138 * t409 * t1040 * t1044;
    (t8633, t8637, t8640, t8644)
}
