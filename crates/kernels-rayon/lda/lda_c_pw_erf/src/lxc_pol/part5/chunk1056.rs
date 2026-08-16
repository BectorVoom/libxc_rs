//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1056/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1056(t1: f64, t1832: f64, t322: f64, t5592: f64, t156: f64, t426: f64, t7129: f64, t7133: f64, t7137: f64, t431: f64, t5594: f64, t7102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19583 = t1832 * t1 * t322;
    let t19584 = t5592 * t19583;
    let t19590 = t426 * t156 * t7129;
    let t19593 = t426 * t156 * t7133;
    let t19604 = t426 * t156 * t7137;
    let t19614 = t431 * t7102 * t5594;
    (t19583, t19584, t19590, t19593, t19604, t19614)
}
