//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 250/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk250<F: Float>(t1091: F, t80: F, t10: F, t104: F, t88: F, t1062: F, t114: F) -> (F, F, F, F) {
    let t1092 = t1091 * t80;
    let t1094 = t104 * t88 * t10;
    let t1095 = t1092 * t1094;
    let t1098 = t114 * t1062;
    (t1092, t1094, t1095, t1098)
}
