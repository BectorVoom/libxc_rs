//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1087/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1087<F: Float>(t5: F, t12912: F, t500: F, t1451: F, t5194: F, t1455: F, t1467: F, t1944: F, t642: F, t10: F, t11013: F, t11021: F, t11024: F, t1941: F, t2192: F, t2195: F, t247: F, t3010: F, t3115: F, t3127: F, t332: F, t395: F, t4687: F, t594: F, t761: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t12913 = t12912 * t500;
    let t12914 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12913;
    let t12915 = t5194 * t1451;
    let t12916 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12915;
    let t12917 = t5194 * t1455;
    let t12918 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12917;
    let t12919 = t5194 * t1467;
    let t12920 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t12919;
    let t12939 = F::cast_from(64.0_f64) * t1944 * t642;
    let t12941 = piecewise3::<F>(t6, F::cast_from(0.0_f64), -F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t2192 * t3010 + F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t2195 * t11013 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t761 * t3127 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t10 * t395 * t332 - F::cast_from(80.0_f64) * t4687 * t11021 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t4687 * t11024 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1941 * t3115 - F::cast_from(32.0_f64) * t594 * t247 + t12939);
    (t12914, t12916, t12918, t12920, t12941)
}
