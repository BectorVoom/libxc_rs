//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1175/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1175(t405: f64, t7847: f64, t103: f64, t1576: f64, t17127: f64, t17129: f64, t17131: f64, t17133: f64, t17138: f64, t17140: f64, t17164: f64, t17166: f64, t19332: f64, t19336: f64, t19340: f64, t19344: f64, t19358: f64, t19362: f64, t19371: f64, t19377: f64, t19385: f64, t19389: f64, t19396: f64, t2060: f64, t3358: f64, t525: f64, t9967: f64) -> f64 {
    let t21144 = t405 * t7847;
    let t21184 = -0.006666666666666667_f64 * t103 * t525 * t19396 + 0.0044444444444444444_f64 * t21144 - 0.08_f64 * t2060 * t1576 * t19389 - 0.006666666666666667_f64 * t103 * t1576 * t19358 + 0.013333333333333334_f64 * t2060 * t1576 * t19362 + 0.16_f64 * t103 * t525 * t19332 + 0.24_f64 * t2060 * t525 * t19336 + 0.04_f64 * t103 * t525 * t19340 - 0.08_f64 * t2060 * t525 * t19344 - 0.08_f64 * t103 * t1576 * t19385 + 0.035555555555555556_f64 * t103 * t3358 * t19371 - 0.006913580246913581_f64 * t103 * t9967 * t19377 + 0.005925925925925926_f64 * t17127 - 0.017777777777777778_f64 * t17129 + 0.2879333333333333_f64 * t17131 - 0.14396666666666666_f64 * t17133 - 0.07198333333333333_f64 * t17138 + 0.023994444444444443_f64 * t17140 + 0.03999074074074074_f64 * t17164 - 0.09597777777777777_f64 * t17166;
    t21184
}
