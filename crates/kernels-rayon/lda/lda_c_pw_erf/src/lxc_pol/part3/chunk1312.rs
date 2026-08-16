//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1312/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1312(t14040: f64, t14042: f64, t14045: f64, t14047: f64, t14050: f64, t14053: f64, t14054: f64, t14055: f64, t14056: f64, t14058: f64, t14066: f64, t14070: f64, t14072: f64) -> f64 {
    let t15121 = -t14040 - t14042 - t14045 - t14047 - t14050 + t14053 + t14054 - t14055 - t14056 - t14058 + t14066 + t14070 - t14072;
    t15121
}
