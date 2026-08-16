//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 270/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk270(t176: f64, t842: f64, t166: f64, t161: f64, t525: f64, t838: f64, t103: f64, t519: f64, t523: f64, t840: f64) -> (f64, f64, f64, f64, f64) {
    let t843 = t842 * t176;
    let t844 = t166 * t843;
    let t846 = t161 * t844 / 30.0_f64;
    let t848 = t525 * t838;
    let t851 = -t519 - 0.035991666666666665_f64 * t840 - t523 - 0.006666666666666667_f64 * t103 * t848;
    (t843, t844, t846, t848, t851)
}
