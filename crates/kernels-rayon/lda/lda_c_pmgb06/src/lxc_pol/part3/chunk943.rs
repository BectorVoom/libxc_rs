//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 943/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk943(t5: f64, t1074: f64, t395: f64, t2128: f64, t642: f64, t1: f64, t1068: f64, t11013: f64, t11021: f64, t2125: f64, t247: f64, t3010: f64, t3115: f64, t3127: f64, t332: f64, t3912: f64, t4486: f64, t4489: f64, t760: f64, t8485: f64, t9: f64, zeta_threshold: f64) -> (f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t11024 = t395 * t1074;
    let t11032 = 32.0_f64 * t2128 * t642;
    let t11034 = piecewise3(t6, 0.0_f64, 40.0_f64 / 81.0_f64 * t8485 * t760 * t3010 - 16.0_f64 / 9.0_f64 * t3912 * t1 * t11013 - 8.0_f64 / 9.0_f64 * t4486 * t3127 + 8.0_f64 / 3.0_f64 * t1068 * t395 * t332 - 8.0_f64 * t4489 * t11021 + 8.0_f64 / 3.0_f64 * t4489 * t11024 + 4.0_f64 / 9.0_f64 * t2125 * t3115 - 16.0_f64 * t9 * t247 + t11032);
    (t11024, t11034)
}
