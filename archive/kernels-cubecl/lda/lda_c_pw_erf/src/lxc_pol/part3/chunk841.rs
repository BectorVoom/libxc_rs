//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 841/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk841<F: Float>(t4041: F, t5164: F, t5169: F, t5172: F, t5174: F, t5177: F, t5179: F, t5181: F, t5182: F, t5183: F, t5186: F, t5188: F, t5190: F, t5192: F, t5194: F, t5196: F, t5198: F) -> F {
    let t5864 = -t5164 + t5169 - t5172 + t5174 + t5177 + t5179 - t5181 + t5182 + t5183 + t4041 - t5186 + t5188 + t5190 + t5192 + t5194 + t5196 - t5198;
    t5864
}
