//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 284/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk284<F: Float>(t905: F, t907: F, t904: F, t27: F, t317: F, t13: F, t334: F, t126: F, t19: F, t299: F, t732: F, t119: F, t473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t908 = t905 * t907;
    let t909 = t904 * t908;
    let t910 = F::cast_from(16.081824322151103_f64) * t909;
    let t911 = t317 * t27;
    let t912 = F::cast_from(1.0_f64) / t911;
    let t913 = t13 * t912;
    let t914 = t905 * t334;
    let t915 = t913 * t914;
    let t916 = F::cast_from(2.0_f64) * t915;
    let t917 = F::cast_from(1.0_f64) / t126;
    let t918 = t917 * t19;
    let t919 = t732 * t299;
    let t920 = t918 * t919;
    let t922 = t119 * t473;
    (t908, t910, t911, t912, t913, t914, t916, t917, t918, t919, t920, t922)
}
