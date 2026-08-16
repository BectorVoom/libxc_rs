//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1119/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1119(t1318: f64, t4688: f64, t4758: f64, t951: f64, t4804: f64, t5409: f64, t3794: f64, t1325: f64, t3859: f64, t5275: f64, t5237: f64, t5265: f64) -> (f64, f64, f64, f64, f64) {
    let t13096 = 16.0_f64 / 15.0_f64 * t1318 * t4758 * t4688 * t951;
    let t13097 = t4804 * t5409;
    let t13098 = 32.0_f64 / 45.0_f64 * t13097;
    let t13099 = t3794 * t5409;
    let t13100 = 32.0_f64 / 45.0_f64 * t13099;
    let t13102 = t1325 * t3859 * t5275;
    let t13103 = 16.0_f64 / 45.0_f64 * t13102;
    let t13105 = t1325 * t5237 * t5265;
    (t13096, t13098, t13100, t13103, t13105)
}
