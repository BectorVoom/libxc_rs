//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1175/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1175<F: Float>(t405: F, t7847: F, t103: F, t1576: F, t17127: F, t17129: F, t17131: F, t17133: F, t17138: F, t17140: F, t17164: F, t17166: F, t19332: F, t19336: F, t19340: F, t19344: F, t19358: F, t19362: F, t19371: F, t19377: F, t19385: F, t19389: F, t19396: F, t2060: F, t3358: F, t525: F, t9967: F) -> F {
    let t21144 = t405 * t7847;
    let t21184 = -F::new(0.006666666666666667) * t103 * t525 * t19396 + F::new(0.0044444444444444444) * t21144 - F::new(0.08) * t2060 * t1576 * t19389 - F::new(0.006666666666666667) * t103 * t1576 * t19358 + F::new(0.013333333333333334) * t2060 * t1576 * t19362 + F::new(0.16) * t103 * t525 * t19332 + F::new(0.24) * t2060 * t525 * t19336 + F::new(0.04) * t103 * t525 * t19340 - F::new(0.08) * t2060 * t525 * t19344 - F::new(0.08) * t103 * t1576 * t19385 + F::new(0.035555555555555556) * t103 * t3358 * t19371 - F::new(0.006913580246913581) * t103 * t9967 * t19377 + F::new(0.005925925925925926) * t17127 - F::new(0.017777777777777778) * t17129 + F::new(0.2879333333333333) * t17131 - F::new(0.14396666666666666) * t17133 - F::new(0.07198333333333333) * t17138 + F::new(0.023994444444444443) * t17140 + F::new(0.03999074074074074) * t17164 - F::new(0.09597777777777777) * t17166;
    t21184
}
