//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 993/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk993(t1147: f64, t934: f64, t940: f64, t623: f64, t8165: f64, t36: f64, t138: f64, t28: f64, t4238: f64, t3689: f64, t642: f64, t950: f64) -> (f64, f64, f64, f64, f64) {
    let t8701 = t934 * t1147;
    let t8702 = t940 * t8701;
    let t8704 = t623 * t8165;
    let t8707 = f64::powf(t36, -2.5_f64);
    let t8710 = t8707 * t28 * t4238 * t138;
    let t8712 = t3689 * t642;
    let t8714 = t950 * t8701;
    (t8702, t8704, t8710, t8712, t8714)
}
