//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 897/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk897(t426: f64, t9024: f64, t435: f64, t97: f64, t3327: f64, t443: f64, t1704: f64, t1710: f64, t3338: f64, t440: f64, t131: f64, t137: f64, t3337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9025 = t426 * t9024;
    let t9037 = 1.0_f64 / t435 / t97;
    let t9051 = t3327 * t443;
    let t9054 = t1704 * t1710;
    let t9059 = t440 * t3338;
    let t9068 = t131 / t3337 / t137;
    (t9025, t9037, t9051, t9054, t9059, t9068)
}
