//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1031/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1031(t3872: f64, t4506: f64, t4508: f64, t1381: f64, t4516: f64, t6728: f64, t3437: f64, t822: f64, t9378: f64, t9380: f64, t1386: f64, t5215: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12078 = 16.0_f64 / 15.0_f64 * t4506 * t4508 * t3872;
    let t12082 = 8.0_f64 / 15.0_f64 * t4506 * t6728 * t4516 * t1381;
    let t12083 = t822 * t3437;
    let t12084 = 16.0_f64 / 405.0_f64 * t12083;
    let t12085 = 4.0_f64 / 45.0_f64 * t9378;
    let t12086 = 4.0_f64 / 45.0_f64 * t9380;
    let t12087 = t5215 * t1386;
    (t12078, t12082, t12084, t12085, t12086, t12087)
}
