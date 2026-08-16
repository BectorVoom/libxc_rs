//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 678/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk678<F: Float>(t4188: F, t4191: F, t4193: F, t4196: F, t4199: F, t4202: F, t4205: F, t4206: F, t4207: F, t4208: F) -> F {
    let t4209 = t4188 + t4191 - t4193 + t4196 + t4199 - t4202 + t4205 - t4206 + t4207 - t4208;
    t4209
}
