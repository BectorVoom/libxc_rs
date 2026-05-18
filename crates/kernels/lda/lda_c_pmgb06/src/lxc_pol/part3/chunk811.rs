//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 811/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk811<F: Float>(t5170: F, t5173: F, t5178: F, t5182: F, t5184: F, t5186: F, t5189: F, t5191: F, t5196: F, t5200: F, t5205: F, t5207: F, t5209: F, t5213: F, t5215: F) -> F {
    let t5666 = t5170 + t5173 + t5178 + t5182 + t5184 + t5186 + t5189 + t5191 + t5196 + t5200 + t5205 + t5207 + t5209 - t5213 + t5215;
    t5666
}
