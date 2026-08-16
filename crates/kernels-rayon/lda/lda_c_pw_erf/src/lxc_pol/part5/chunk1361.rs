//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1361/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1361(t11266: f64, t14440: f64, t14444: f64, t14448: f64, t15297: f64, t20097: f64, t20098: f64, t20138: f64, t20199: f64, t20200: f64, t20230: f64, t20246: f64, t20550: f64, t20586: f64, t20628: f64, t23118: f64, t23358: f64, t312: f64, t8527: f64, t8533: f64, t8536: f64, t8539: f64, t8542: f64, t8716: f64, t8733: f64, t8737: f64, t8740: f64) -> f64 {
    let t23363 = -t14440 + t8527 + t20097 + t8533 - t8536 + t8539 - t8542 - t11266 - t14444 - t20098 - (t20138 + t20230 + t20246 + t20550 + t20586 + t20628 + t23118 + t23358) * t312 + t15297 + t14448 - t20199 - t20200 + t8733 - t8716 - t8737 + t8740;
    t23363
}
