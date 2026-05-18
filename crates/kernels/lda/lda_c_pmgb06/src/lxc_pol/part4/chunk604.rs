//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 604/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk604<F: Float>(t2695: F, t64: F, t38: F, t2448: F, t56: F, t370: F) -> (F, F, F, F, F) {
    let t2696 = t64 * t2695;
    let t2698 = F::new(5.84605) * t38 * t2696;
    let t2699 = t56 * t2448;
    let t2701 = F::new(2.923025) * t38 * t2699;
    let t2703 = t370 * t2695;
    (t2696, t2698, t2699, t2701, t2703)
}
