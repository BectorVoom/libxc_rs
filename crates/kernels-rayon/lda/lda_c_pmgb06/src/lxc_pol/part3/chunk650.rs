//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 650/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk650(t5: f64, t12: f64, t1068: f64, t332: f64, t1074: f64, t3010: f64, t3115: f64, t3912: f64, t9: f64, t598: f64, t1079: f64, t337: f64, t1083: f64, t14: f64, t2912: f64, t2938: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t3915 = t1068 * t332;
    let t3921 = piecewise3(t6, 0.0_f64, -8.0_f64 / 27.0_f64 * t3912 * t3010 + 4.0_f64 / 3.0_f64 * t3915 * t1074 + 4.0_f64 / 3.0_f64 * t9 * t3115);
    let t3922 = 1.0_f64 / t598;
    let t3925 = t1079 * t337;
    let t3931 = piecewise3(t13, 0.0_f64, -8.0_f64 / 27.0_f64 * t3922 * t2912 + 4.0_f64 / 3.0_f64 * t3925 * t1083 + 4.0_f64 / 3.0_f64 * t14 * t2938);
    (t3921, t3922, t3931)
}
