//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 284/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk284(t905: f64, t907: f64, t904: f64, t27: f64, t317: f64, t13: f64, t334: f64, t126: f64, t19: f64, t299: f64, t732: f64, t119: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t908 = t905 * t907;
    let t909 = t904 * t908;
    let t910 = 16.081824322151103_f64 * t909;
    let t911 = t317 * t27;
    let t912 = 1.0_f64 / t911;
    let t913 = t13 * t912;
    let t914 = t905 * t334;
    let t915 = t913 * t914;
    let t916 = 2.0_f64 * t915;
    let t917 = 1.0_f64 / t126;
    let t918 = t917 * t19;
    let t919 = t732 * t299;
    let t920 = t918 * t919;
    let t922 = t119 * t473;
    (t908, t910, t911, t912, t913, t914, t916, t917, t918, t919, t920, t922)
}
