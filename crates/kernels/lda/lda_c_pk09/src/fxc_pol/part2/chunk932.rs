//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 932/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk932<F: Float>(t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t7149: F, t7150: F, t7154: F) -> (F,) {
    let t11551 = 0.2946275542389858 * t11066 + 0.5892551084779716 * t10959 + 2.9540870317630623 * t11529 - 2.9540870317630623 * t11532 - 2.9540870317630623 * t11535 + 4.431130547644593 * t11539 - 2.9540870317630623 * t11542 + 0.2946275542389858 * t11076 + t7149 + 0.0982091847463286 * t11073 + t7154 - 0.0982091847463286 * t6337 - 0.2946275542389858 * t6323 + 0.9846956772543541 * t6550 + t7150 - 0.9846956772543541 * t6508 + 0.0982091847463286 * t6467;
    (t11551,)
}
