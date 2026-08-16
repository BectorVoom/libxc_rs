//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1056/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1056(t12373: f64, t4488: f64, t4501: f64, t12337: f64, t12339: f64, t12341: f64, t12345: f64, t12348: f64, t12351: f64, t12355: f64, t12357: f64, t12361: f64, t12367: f64, t12369: f64, t12372: f64) -> (f64, f64) {
    let t12376 = 4.0_f64 / 9.0_f64 * t4488 * t4501 * t12373;
    let t12377 = t12337 + t12339 + t12341 + t12345 + t12348 + t12351 + t12355 + t12357 + t12361 + t12367 - t12369 - t12372 - t12376;
    (t12376, t12377)
}
