//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 817/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk817<F: Float>(t2574: F, t822: F, t1385: F, t439: F, t2545: F, t851: F, t1380: F, t493: F, t2604: F, t5118: F, t137: F, t132: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7710 = t2574 * t822;
    let t7711 = t1385 * t7710;
    let t7713 = F::new(2.0) / F::new(15.0) * t439 * t7711;
    let t7714 = t2545 * t851;
    let t7715 = t1380 * t7714;
    let t7717 = F::new(2.0) / F::new(15.0) * t493 * t7715;
    let t7718 = t5118 * t2604;
    let t7719 = t137 * t7718;
    let t7721 = t132 * t7719 / F::new(5.0);
    (t7710, t7711, t7713, t7714, t7715, t7717, t7718, t7719, t7721)
}
