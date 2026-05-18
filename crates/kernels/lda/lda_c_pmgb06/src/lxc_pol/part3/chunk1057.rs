//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1057/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1057<F: Float>(t12518: F, t12524: F, t12527: F, t12534: F, t12542: F, t12545: F, t12550: F, t12553: F, t12557: F, t12561: F, t12566: F, t12571: F) -> F {
    let t12572 = -t12518 + t12524 - t12527 - t12534 + t12542 - t12545 + t12550 + t12553 + t12557 + t12561 - t12566 + t12571;
    t12572
}
