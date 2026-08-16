//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1397/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1397<F: Float>(t16124: F, t16126: F, t16130: F, t16132: F, t16135: F, t16136: F, t16138: F, t16139: F, t16140: F, t16141: F, t16145: F, t16149: F, t16151: F, t16153: F, t16157: F) -> F {
    let t18208 = t16124 + t16126 + t16130 - t16132 + t16135 - t16136 + t16138 + t16139 + t16140 + t16141 - t16145 + t16149 - t16151 - t16153 + t16157;
    t18208
}
