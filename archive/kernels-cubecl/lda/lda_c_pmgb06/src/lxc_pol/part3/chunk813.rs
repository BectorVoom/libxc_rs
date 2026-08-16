//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 813/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk813<F: Float>(t5263: F, t5266: F, t5270: F, t5275: F, t5279: F, t5284: F, t5286: F, t5289: F, t5293: F, t5297: F, t5301: F, t5304: F, t5307: F, t5309: F, t5311: F, t5315: F) -> F {
    let t5669 = t5263 + t5266 - t5270 - t5275 - t5279 - t5284 - t5286 - t5289 - t5293 - t5297 + t5301 - t5304 + t5307 + t5309 + t5311 + t5315;
    t5669
}
