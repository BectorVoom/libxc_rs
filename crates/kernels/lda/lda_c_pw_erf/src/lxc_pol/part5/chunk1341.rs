//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1341/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1341<F: Float>(t22210: F, t22212: F, t22214: F, t22216: F, t22218: F, t22222: F, t22225: F, t22228: F, t22231: F, t22234: F, t22237: F, t22239: F, t22243: F) -> F {
    let t23296 = t22210 + t22212 + t22214 + t22216 + t22218 - t22222 - t22225 + t22228 + t22231 - t22234 + t22237 - t22239 - t22243;
    t23296
}
