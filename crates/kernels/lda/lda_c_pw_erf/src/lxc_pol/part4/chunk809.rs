//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 809/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk809<F: Float>(t5200: F, t5202: F, t5203: F, t5204: F, t5213: F, t5217: F, t5224: F, t5228: F, t5233: F, t5236: F, t5240: F, t5242: F, t5246: F, t5249: F, t5253: F, t5259: F, t5263: F) -> (F,) {
    let t5867 = t5200 + t5202 + t5203 + t5204 + t5213 + t5217 - t5224 + t5228 + t5233 - t5236 + t5240 - t5242 - t5246 + t5249 + t5253 + t5259 - t5263;
    (t5867,)
}
