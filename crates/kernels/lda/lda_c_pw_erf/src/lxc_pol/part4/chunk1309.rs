//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1309/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1309<F: Float>(t17128: F, t17130: F, t17132: F, t17134: F, t17140: F, t17143: F, t17146: F, t17149: F, t17151: F, t17154: F, t17157: F, t17159: F, t17163: F, t17167: F, t17169: F, t17172: F, t17173: F) -> (F,) {
    let t19239 = -t17128 - t17130 - t17132 + t17134 + t17140 + t17143 - t17146 - t17149 + t17151 + t17154 + t17157 - t17159 - t17163 - t17167 - t17169 - t17172 - t17173;
    (t19239,)
}
