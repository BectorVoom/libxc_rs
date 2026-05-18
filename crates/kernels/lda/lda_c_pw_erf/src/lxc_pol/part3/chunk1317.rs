//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1317/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1317<F: Float>(t14220: F, t14222: F, t14227: F, t14230: F, t14233: F, t14236: F, t14239: F, t14243: F, t14246: F, t14248: F, t14250: F, t14252: F, t14256: F) -> F {
    let t15159 = -t14220 - t14222 - t14227 + t14230 + t14233 - t14236 - t14239 + t14243 + t14246 + t14248 + t14250 - t14252 - t14256;
    t15159
}
