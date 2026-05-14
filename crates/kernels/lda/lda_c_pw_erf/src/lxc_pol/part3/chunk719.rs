//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 719/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk719<F: Float>(t2114: F, t2127: F, t2131: F, t4060: F, t4064: F, t4041: F, t4215: F, t4217: F, t5181: F, t5182: F, t5183: F, t5186: F, t5188: F, t5190: F, t5192: F, t5194: F, t5196: F, t5198: F) -> (F, F, F, F, F) {
    let t5200 = 16.0 / 45.0 * t2114 * t2127;
    let t5202 = 8.0 / 15.0 * t2114 * t2131;
    let t5203 = 8.0 / 135.0 * t4060;
    let t5204 = 8.0 / 81.0 * t4064;
    let t5205 = -t5181 + t5182 + t5183 + t4041 - t5186 + t5188 + t5190 + t5192 + t5194 + t5196 - t5198 + t5200 + t5202 + t5203 + t5204 + t4215 + t4217;
    (t5200, t5202, t5203, t5204, t5205)
}
