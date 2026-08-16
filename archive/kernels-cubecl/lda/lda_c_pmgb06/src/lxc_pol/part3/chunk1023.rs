//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1023/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1023<F: Float>(t12129: F, t12131: F, t12135: F, t12138: F, t12142: F, t12145: F, t12149: F, t12153: F, t12159: F, t12164: F, t12168: F, t12170: F) -> F {
    let t12171 = -t12129 + t12131 + t12135 + t12138 + t12142 + t12145 + t12149 + t12153 + t12159 + t12164 - t12168 - t12170;
    t12171
}
