//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 957/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk957<F: Float>(t5: F, t342: F, t5870: F, t370: F, t38: F, t11227: F, t8281: F, t2195: F, t642: F, t1: F, t11013: F, t11021: F, t11024: F, t1212: F, t2192: F, t247: F, t3010: F, t3115: F, t3127: F, t330: F, t332: F, t3537: F, t395: F, t4363: F, t4366: F, t760: F, t8119: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t11231 = t5870 * t342;
    let t11234 = t38 * t370;
    let t11236 = F::cast_from(52.61445_f64) * t11234 * t11231;
    let t11237 = t8281 * t11227;
    let t11259 = F::cast_from(16.0_f64) * t2195 * t642;
    let t11261 = piecewise3::<F>(t6, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t8119 * t760 * t3010 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3537 * t1 * t11013 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4363 * t3127 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1212 * t395 * t332 + F::cast_from(4.0_f64) * t4366 * t11021 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4366 * t11024 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2192 * t3115 - F::cast_from(8.0_f64) * t330 * t247 + t11259);
    (t11231, t11236, t11237, t11261)
}
