//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 611/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk611(t1623: f64, t405: f64, t3082: f64, t3084: f64, t3086: f64, t3088: f64, t3095: f64, t3101: f64, t3106: f64, t3110: f64, t3113: f64, t3118: f64) -> (f64, f64) {
    let t3428 = t405 * t1623;
    let t3440 = -0.02666666666666667_f64 * t3428 - 0.07198333333333333_f64 * t3086 + 0.14396666666666666_f64 * t3101 - 0.07198333333333333_f64 * t3106 - 0.21595_f64 * t3110 + 0.21595_f64 * t3113 - 0.047988888888888886_f64 * t3082 + 0.035991666666666665_f64 * t3088 + 0.023994444444444443_f64 * t3084 - 0.03999074074074074_f64 * t3095 - 0.035991666666666665_f64 * t3118;
    (t3428, t3440)
}
