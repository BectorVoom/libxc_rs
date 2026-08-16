//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1126/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1126(t16964: f64, t16966: f64, t16968: f64, t16970: f64, t16992: f64, t17004: f64, t17006: f64, t432: f64, t7736: f64, t486: f64, t7808: f64, t10185: f64, t161: f64, t166: f64, t7806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20515 = 2.0_f64 / 45.0_f64 * t16964;
    let t20516 = 2.0_f64 / 45.0_f64 * t16966;
    let t20517 = 2.0_f64 / 27.0_f64 * t16968;
    let t20518 = 2.0_f64 / 27.0_f64 * t16970;
    let t20519 = 4.0_f64 / 15.0_f64 * t16992;
    let t20520 = 2.0_f64 / 5.0_f64 * t17004;
    let t20521 = 4.0_f64 / 15.0_f64 * t17006;
    let t20523 = t432 * t7736 / 10.0_f64;
    let t20525 = t486 * t7808 / 5.0_f64;
    let t20529 = t161 * t166 * t10185 * t7806 / 5.0_f64;
    (t20515, t20516, t20517, t20518, t20519, t20520, t20521, t20523, t20525, t20529)
}
