//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 911/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk911(t1244: f64, t2061: f64, t539: f64, t1250: f64, t1184: f64, t56: f64, t174: f64, t177: f64, t1191: f64, t191: f64, t187: f64, t190: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9762 = t1244 * t1244;
    let t9763 = 1.0_f64 / t9762;
    let t9772 = t2061 * t539;
    let t9777 = 1.0_f64 / t1244 / t1250;
    let t9810 = t1184 * t56;
    let t9812 = t174 * t9810 * t177;
    let t9813 = 0.3732469135802469_f64 * t9812;
    let t9821 = t1191 * t191;
    let t9824 = 0.10864197530864197_f64 * t190 * t9821 * t187;
    (t9763, t9772, t9777, t9810, t9812, t9813, t9821, t9824)
}
