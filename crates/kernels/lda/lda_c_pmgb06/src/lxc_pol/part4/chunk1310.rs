//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1310/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1310<F: Float>(t103: F, t13399: F, t13407: F, t14162: F, t14170: F, t14181: F, t14183: F, t14185: F, t14187: F, t14189: F, t16359: F, t3358: F, t9530: F, t9532: F, t9552: F) -> F {
    let t17245 = F::cast_from(0.035555555555555556_f64) * t103 * t3358 * t16359 + F::cast_from(1.135737037037037_f64) * t13399 + F::cast_from(0.07464938271604939_f64) * t13407 + F::cast_from(0.2725925925925926_f64) * t14162 + F::cast_from(0.03950617283950617_f64) * t14170 + F::cast_from(0.05925925925925926_f64) * t14181 - F::cast_from(0.009876543209876543_f64) * t14183 - F::cast_from(0.017777777777777778_f64) * t14185 + F::cast_from(0.07111111111111111_f64) * t14187 + F::cast_from(0.002962962962962963_f64) * t14189 - F::cast_from(0.015996296296296297_f64) * t9530 - F::cast_from(0.010664197530864198_f64) * t9532 + F::cast_from(0.07464938271604939_f64) * t9552;
    t17245
}
