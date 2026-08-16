//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1310/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1310(t103: f64, t13399: f64, t13407: f64, t14162: f64, t14170: f64, t14181: f64, t14183: f64, t14185: f64, t14187: f64, t14189: f64, t16359: f64, t3358: f64, t9530: f64, t9532: f64, t9552: f64) -> f64 {
    let t17245 = 0.035555555555555556_f64 * t103 * t3358 * t16359 + 1.135737037037037_f64 * t13399 + 0.07464938271604939_f64 * t13407 + 0.2725925925925926_f64 * t14162 + 0.03950617283950617_f64 * t14170 + 0.05925925925925926_f64 * t14181 - 0.009876543209876543_f64 * t14183 - 0.017777777777777778_f64 * t14185 + 0.07111111111111111_f64 * t14187 + 0.002962962962962963_f64 * t14189 - 0.015996296296296297_f64 * t9530 - 0.010664197530864198_f64 * t9532 + 0.07464938271604939_f64 * t9552;
    t17245
}
