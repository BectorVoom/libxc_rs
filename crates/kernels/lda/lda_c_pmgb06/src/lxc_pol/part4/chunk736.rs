//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 736/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk736<F: Float>(t439: F, t5238: F, t5196: F, t5200: F, t5205: F, t5207: F, t5209: F, t5213: F, t5215: F, t5217: F, t5219: F, t5222: F, t5224: F, t5228: F, t5230: F, t5235: F, t5237: F) -> (F, F) {
    let t5240 = 2.0 / 45.0 * t439 * t5238;
    let t5241 = t5196 + t5200 + t5205 + t5207 + t5209 - t5213 + t5215 + t5217 + t5219 + t5222 - t5224 - t5228 - t5230 - t5235 - t5237 - t5240;
    (t5240, t5241)
}
