//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 812/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk812<F: Float>(t5217: F, t5219: F, t5222: F, t5224: F, t5228: F, t5230: F, t5235: F, t5237: F, t5240: F, t5244: F, t5247: F, t5250: F, t5252: F, t5256: F, t5259: F) -> F {
    let t5668 = t5217 + t5219 + t5222 - t5224 - t5228 - t5230 - t5235 - t5237 - t5240 - t5244 - t5247 - t5250 + t5252 + t5256 + t5259;
    t5668
}
