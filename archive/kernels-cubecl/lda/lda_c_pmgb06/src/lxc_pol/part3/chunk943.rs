//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 943/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk943<F: Float>(t5: F, t1074: F, t395: F, t2128: F, t642: F, t1: F, t1068: F, t11013: F, t11021: F, t2125: F, t247: F, t3010: F, t3115: F, t3127: F, t332: F, t3912: F, t4486: F, t4489: F, t760: F, t8485: F, t9: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t11024 = t395 * t1074;
    let t11032 = F::cast_from(32.0_f64) * t2128 * t642;
    let t11034 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t8485 * t760 * t3010 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3912 * t1 * t11013 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4486 * t3127 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1068 * t395 * t332 - F::cast_from(8.0_f64) * t4489 * t11021 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4489 * t11024 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2125 * t3115 - F::cast_from(16.0_f64) * t9 * t247 + t11032);
    (t11024, t11034)
}
