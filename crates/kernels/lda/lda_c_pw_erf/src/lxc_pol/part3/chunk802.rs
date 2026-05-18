//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 802/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk802<F: Float>(t4513: F, t4542: F, t4560: F, t4650: F, t4741: F, t4798: F, t4853: F, t4910: F, t4964: F, t5062: F, t5180: F, t5205: F, t5254: F, t5325: F, t5370: F, t5429: F) -> F {
    let t5433 = t4513 + t4542 + t4560 + t4650 + t4741 + t4798 + t4853 + t4910 + t4964 + t5062 + t5180 + t5205 + t5254 + t5325 + t5370 + t5429;
    t5433
}
