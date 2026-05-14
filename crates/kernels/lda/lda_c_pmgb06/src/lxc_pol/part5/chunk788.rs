//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 788/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk788<F: Float>(t138: F, t3875: F, t3885: F, t109: F, t3665: F, t3670: F, t1179: F, t621: F, t634: F, t1036: F, t3878: F, t1040: F, t1044: F, t409: F, t1018: F, t3698: F) -> (F, F, F, F, F, F) {
    let t8629 = 0.4274 * t138 * t3885 * t3875;
    let t8633 = 36.84616320282908 * t138 * t109 * t3665 * t3670;
    let t8637 = 0.22161481481481482 * t138 * t1179 * t621 * t634;
    let t8640 = 0.14246666666666666 * t138 * t3878 * t1036;
    let t8644 = 2.2911460125803966 * t138 * t409 * t1040 * t1044;
    let t8647 = 0.07123333333333333 * t138 * t1018 * t3698;
    (t8629, t8633, t8637, t8640, t8644, t8647)
}
