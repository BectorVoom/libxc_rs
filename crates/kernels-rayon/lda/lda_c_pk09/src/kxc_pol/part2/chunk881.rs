//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 881/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk881(t133: f64, t2335: f64, t131: f64, t4379: f64, t4380: f64, t4382: f64, t4384: f64, t4386: f64, t4388: f64, t4391: f64, t4397: f64, t8525: f64, t8527: f64, t8529: f64, t8531: f64, t8533: f64, t933: f64) -> f64 {
    let t9245 = t133 * t2335;
    let t9246 = t131 * t9245;
    let t9256 = 0.037002892246025966_f64 * t8525 + 0.02466859483068398_f64 * t8527 - 0.02466859483068398_f64 * t8529 + 0.02466859483068398_f64 * t8531 + t933 * t9246 / 36.0_f64 + 0.14975624337724558_f64 * t8533 + t4379 - t4380 / 9.0_f64 - t4382 / 9.0_f64 + t4384 / 9.0_f64 - t4386 / 9.0_f64 - t4388 / 9.0_f64 + t4391 / 9.0_f64 - t4397;
    t9256
}
