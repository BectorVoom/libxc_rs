//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1165/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1165(t1894: f64, t3220: f64, t1898: f64, t1902: f64, t1423: f64, t5287: f64, t5226: f64, t5254: f64, t5211: f64, t5295: f64, t13892: f64, t13894: f64, t13896: f64, t13899: f64, t13904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13905 = t3220 * t1894;
    let t13906 = 4.0_f64 / 45.0_f64 * t13905;
    let t13907 = t3220 * t1898;
    let t13908 = 8.0_f64 / 45.0_f64 * t13907;
    let t13909 = t3220 * t1902;
    let t13910 = 4.0_f64 / 27.0_f64 * t13909;
    let t13911 = t1423 * t5287;
    let t13912 = 4.0_f64 / 45.0_f64 * t13911;
    let t13913 = t1423 * t5226;
    let t13914 = 8.0_f64 / 45.0_f64 * t13913;
    let t13915 = t1423 * t5254;
    let t13916 = 4.0_f64 / 27.0_f64 * t13915;
    let t13917 = t5211 * t5295;
    let t13918 = 2.0_f64 / 9.0_f64 * t13917;
    let t13919 = -t13892 - t13894 - t13896 + t13899 + t13904 - t13906 - t13908 + t13910 - t13912 - t13914 + t13916 + t13918;
    (t13906, t13908, t13910, t13912, t13914, t13916, t13918, t13919)
}
