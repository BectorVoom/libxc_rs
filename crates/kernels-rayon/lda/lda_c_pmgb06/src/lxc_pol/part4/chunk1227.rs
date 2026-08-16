//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1227/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1227(t2493: f64, t3220: f64, t132: f64, t1547: f64, t2605: f64, t2088: f64, t1601: f64, t161: f64, t166: f64, t4839: f64, t497: f64, t843: f64) -> (f64, f64, f64, f64) {
    let t16158 = t3220 * t2493;
    let t16159 = 8.0_f64 / 135.0_f64 * t16158;
    let t16161 = t132 * t1547 * t2605;
    let t16162 = 2.0_f64 / 135.0_f64 * t16161;
    let t16163 = t2088 * t2088;
    let t16167 = 2.0_f64 / 15.0_f64 * t161 * t166 * t1601 * t16163;
    let t16171 = 4.0_f64 / 45.0_f64 * t161 * t4839 * t843 * t497;
    (t16159, t16162, t16167, t16171)
}
