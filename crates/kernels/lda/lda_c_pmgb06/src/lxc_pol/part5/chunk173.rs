//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 173/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk173<F: Float>(t153: F, t458: F, t137: F, t132: F, t152: F) -> (F, F, F, F, F) {
    let t459 = t458 * t153;
    let t460 = t137 * t459;
    let t462 = t132 * t460 / 30.0;
    let t463 = t152 * t152;
    let t464 = 1.0 / t463;
    (t459, t460, t462, t463, t464)
}
