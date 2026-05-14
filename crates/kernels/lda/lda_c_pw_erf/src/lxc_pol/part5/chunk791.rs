//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 791/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk791<F: Float>(t143: F, t2211: F, t279: F, t405: F, t4129: F, t4136: F, t4140: F, t4144: F, t5933: F, t7072: F, t7077: F, t777: F, t7913: F, t8001: F, t8004: F, t8068: F) -> (F,) {
    let t8074 = -0.03592270203076383 * t7072 - 2.0 * t777 * t8001 + 9.0 * t2211 * t8004 + 0.05987117005127304 * t7077 + t8068 * t279 + 3.0 * t405 * t143 * t7913 - 5.4655730795145296e-05 * t5933 - t4129 + t4136 - t4140 - t4144;
    (t8074,)
}
