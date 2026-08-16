//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1087/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1087(t5: f64, t12912: f64, t500: f64, t1451: f64, t5194: f64, t1455: f64, t1467: f64, t1944: f64, t642: f64, t10: f64, t11013: f64, t11021: f64, t11024: f64, t1941: f64, t2192: f64, t2195: f64, t247: f64, t3010: f64, t3115: f64, t3127: f64, t332: f64, t395: f64, t4687: f64, t594: f64, t761: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t12913 = t12912 * t500;
    let t12914 = 4.0_f64 / 45.0_f64 * t12913;
    let t12915 = t5194 * t1451;
    let t12916 = 4.0_f64 / 45.0_f64 * t12915;
    let t12917 = t5194 * t1455;
    let t12918 = 2.0_f64 / 45.0_f64 * t12917;
    let t12919 = t5194 * t1467;
    let t12920 = 2.0_f64 / 27.0_f64 * t12919;
    let t12939 = 64.0_f64 * t1944 * t642;
    let t12941 = piecewise3(t6, 0.0_f64, -80.0_f64 / 81.0_f64 * t2192 * t3010 + 160.0_f64 / 9.0_f64 * t2195 * t11013 + 80.0_f64 / 9.0_f64 * t761 * t3127 + 80.0_f64 / 3.0_f64 * t10 * t395 * t332 - 80.0_f64 * t4687 * t11021 + 80.0_f64 / 3.0_f64 * t4687 * t11024 + 40.0_f64 / 9.0_f64 * t1941 * t3115 - 32.0_f64 * t594 * t247 + t12939);
    (t12914, t12916, t12918, t12920, t12941)
}
