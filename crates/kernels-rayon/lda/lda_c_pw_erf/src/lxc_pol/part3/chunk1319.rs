//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1319/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1319(t108: f64, t14260: f64, t14262: f64, t14264: f64, t14271: f64, t14275: f64, t14278: f64, t14283: f64, t14285: f64, t14287: f64, t14289: f64, t14291: f64, t14293: f64, t15198: f64, t267: f64) -> f64 {
    let t15202 = t14260 + t14262 + t14264 + t14271 - t14275 - t14278 + t14283 + t14285 + t14287 - t14289 + t14291 + t14293 - t15198 * t108 * t267 / 15.0_f64;
    t15202
}
