//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1222/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1222(t188: f64, t4463: f64, t539: f64, t1409: f64, t1798: f64, t3023: f64, t794: f64, t11589: f64, t14005: f64, t14007: f64, t14010: f64, t14012: f64, t14014: f64, t14016: f64, t14018: f64, t14020: f64, t183: f64) -> f64 {
    let t14478 = t4463 * t539 * t188;
    let t14481 = t1798 * t1409 * t188;
    let t14482 = 4.0_f64 * t14481;
    let t14484 = t794 * t3023 * t188;
    let t14486 = -t14005 - t14007 - t14010 - t14012 - t14014 + t14016 - t14018 - t14020 + 4.0_f64 / 3.0_f64 * t11589 * t183 * t188 + 4.0_f64 * t14478 + t14482 + 4.0_f64 / 3.0_f64 * t14484;
    t14486
}
