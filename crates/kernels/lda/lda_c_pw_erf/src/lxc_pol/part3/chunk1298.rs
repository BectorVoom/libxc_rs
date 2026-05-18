//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1298/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1298<F: Float>(t13361: F, t13364: F, t13367: F, t13371: F, t13373: F, t13376: F, t13387: F, t13390: F, t13392: F, t13396: F, t13398: F, t13400: F, t13402: F) -> F {
    let t15082 = -t13361 - t13364 + t13367 - t13371 - t13373 + t13376 - t13387 + t13390 + t13392 + t13396 + t13398 + t13400 + t13402;
    t15082
}
