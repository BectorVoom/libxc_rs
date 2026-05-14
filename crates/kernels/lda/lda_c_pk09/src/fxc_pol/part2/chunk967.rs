//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 967/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk967<F: Float>(t10954: F, t10959: F, t10962: F, t10966: F, t11062: F, t11066: F, t11070: F, t11073: F, t11076: F, t6323: F, t6327: F, t6337: F, t6467: F, t6836: F, t6838: F, t6844: F) -> (F,) {
    let t12133 = t6836 - 0.9421211958699838 * t6323 + t6838 + 0.9421211958699838 * t6327 - 0.9421211958699838 * t10954 + 1.8842423917399675 * t10959 - 0.3140403986233279 * t10962 - 0.9421211958699838 * t10966 - 0.9421211958699838 * t11062 - 0.3140403986233279 * t6337 - t6844 + 0.3140403986233279 * t6467 + 0.9421211958699838 * t11066 - 0.9421211958699838 * t11070 + 0.3140403986233279 * t11073 + 0.9421211958699838 * t11076;
    (t12133,)
}
