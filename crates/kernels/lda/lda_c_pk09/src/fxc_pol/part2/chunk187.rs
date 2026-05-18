//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 187/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk187<F: Float>(t48: F, t92: F, t56: F, t14: F, t623: F, t54: F, t633: F) -> (F, F, F) {
    let t640 = t92 * t48;
    let t642 = t56 * t640 / F::new(3.0);
    let t643 = t14 * t623;
    let t645 = t92 * t54;
    let t647 = t56 * t645 / F::new(3.0);
    let t648 = t14 * t633;
    let t650 = t56 * t643 + t56 * t648 + t642 + t647;
    (t642, t647, t650)
}
