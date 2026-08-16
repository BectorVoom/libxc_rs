//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 337/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk337(t1156: f64, t123: f64, t199: f64, t566: f64, t722: f64, t81: f64, t1072: f64, t1105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1158 = t123 * t1156 * t199;
    let t1161 = t123 * t722 * t566;
    let t1163 = 2.0_f64 * t81;
    let t1164 = 8.0_f64 * t1072;
    let t1165 = 6.0_f64 * t1105;
    let t1166 = -t1163 + t1164 - t1165;
    (t1158, t1161, t1163, t1164, t1165, t1166)
}
