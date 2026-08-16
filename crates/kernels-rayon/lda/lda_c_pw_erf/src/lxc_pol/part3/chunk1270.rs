//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1270/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1270(t12367: f64, t12369: f64, t12372: f64, t12376: f64, t12383: f64, t12386: f64, t12392: f64, t12395: f64, t12398: f64, t12402: f64, t12406: f64, t12408: f64, t12410: f64) -> f64 {
    let t15008 = t12367 - t12369 - t12372 - t12376 - t12383 - t12386 + t12392 + t12395 + t12398 + t12402 - t12406 - t12408 - t12410;
    t15008
}
