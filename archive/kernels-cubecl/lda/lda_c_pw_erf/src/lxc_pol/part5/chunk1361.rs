//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1361/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1361<F: Float>(t11266: F, t14440: F, t14444: F, t14448: F, t15297: F, t20097: F, t20098: F, t20138: F, t20199: F, t20200: F, t20230: F, t20246: F, t20550: F, t20586: F, t20628: F, t23118: F, t23358: F, t312: F, t8527: F, t8533: F, t8536: F, t8539: F, t8542: F, t8716: F, t8733: F, t8737: F, t8740: F) -> F {
    let t23363 = -t14440 + t8527 + t20097 + t8533 - t8536 + t8539 - t8542 - t11266 - t14444 - t20098 - (t20138 + t20230 + t20246 + t20550 + t20586 + t20628 + t23118 + t23358) * t312 + t15297 + t14448 - t20199 - t20200 + t8733 - t8716 - t8737 + t8740;
    t23363
}
