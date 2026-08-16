//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 584/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk584(t1697: f64, t3222: f64, t10: f64, t427: f64, t474: f64, t426: f64, t156: f64, t1682: f64, t259: f64, t47: f64, t1558: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3223 = t1697 * t3222;
    let t3224 = t10 * t3223;
    let t3227 = t474 * t427;
    let t3228 = t426 * t3227;
    let t3230 = t156 * t1682;
    let t3231 = t426 * t3230;
    let t3234 = 1.0_f64 / t47 / t259;
    let t3237 = t1558 * t348;
    (t3223, t3224, t3227, t3228, t3230, t3231, t3234, t3237)
}
