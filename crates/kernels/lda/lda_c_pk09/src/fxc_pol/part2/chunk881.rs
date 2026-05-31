//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 881/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk881<F: Float>(t133: F, t2335: F, t131: F, t4379: F, t4380: F, t4382: F, t4384: F, t4386: F, t4388: F, t4391: F, t4397: F, t8525: F, t8527: F, t8529: F, t8531: F, t8533: F, t933: F) -> F {
    let t9245 = t133 * t2335;
    let t9246 = t131 * t9245;
    let t9256 = F::cast_from(0.037002892246025966_f64) * t8525 + F::cast_from(0.02466859483068398_f64) * t8527 - F::cast_from(0.02466859483068398_f64) * t8529 + F::cast_from(0.02466859483068398_f64) * t8531 + t933 * t9246 / F::cast_from(36.0_f64) + F::cast_from(0.14975624337724558_f64) * t8533 + t4379 - t4380 / F::cast_from(9.0_f64) - t4382 / F::cast_from(9.0_f64) + t4384 / F::cast_from(9.0_f64) - t4386 / F::cast_from(9.0_f64) - t4388 / F::cast_from(9.0_f64) + t4391 / F::cast_from(9.0_f64) - t4397;
    t9256
}
