//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 957/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk957(t5: f64, t342: f64, t5870: f64, t370: f64, t38: f64, t11227: f64, t8281: f64, t2195: f64, t642: f64, t1: f64, t11013: f64, t11021: f64, t11024: f64, t1212: f64, t2192: f64, t247: f64, t3010: f64, t3115: f64, t3127: f64, t330: f64, t332: f64, t3537: f64, t395: f64, t4363: f64, t4366: f64, t760: f64, t8119: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t11231 = t5870 * t342;
    let t11234 = t38 * t370;
    let t11236 = 52.61445_f64 * t11234 * t11231;
    let t11237 = t8281 * t11227;
    let t11259 = 16.0_f64 * t2195 * t642;
    let t11261 = piecewise3(t6, 0.0_f64, -56.0_f64 / 81.0_f64 * t8119 * t760 * t3010 + 16.0_f64 / 9.0_f64 * t3537 * t1 * t11013 + 8.0_f64 / 9.0_f64 * t4363 * t3127 - 4.0_f64 / 3.0_f64 * t1212 * t395 * t332 + 4.0_f64 * t4366 * t11021 - 4.0_f64 / 3.0_f64 * t4366 * t11024 - 2.0_f64 / 9.0_f64 * t2192 * t3115 - 8.0_f64 * t330 * t247 + t11259);
    (t11231, t11236, t11237, t11261)
}
