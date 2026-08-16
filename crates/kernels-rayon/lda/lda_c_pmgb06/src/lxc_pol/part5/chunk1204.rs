//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1204/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1204(t273: f64, t698: f64, t7402: f64, t11110: f64, t11113: f64, t11115: f64, t11117: f64, t11119: f64, t11123: f64, t11124: f64, t21770: f64, t248: f64, t285: f64, t8724: f64, t8727: f64, t8733: f64, t8737: f64, t8738: f64, t8743: f64, t8746: f64) -> f64 {
    let t21787 = t7402 * t273 * t698;
    let t21796 = 103.89515463408878_f64 * t8724 - 36.0_f64 * t11110 - t11113 + t8727 - 0.5848223622634646_f64 * t21787 + t248 * t21770 * t285 + t8733 - 0.09759223170271566_f64 * t11115 - 0.06506148780181044_f64 * t11117 + 1.4447919941302971_f64 * t11119 + t11123 + 0.04879611585135783_f64 * t11124 - t8737 - 3.5089341735807875_f64 * t8738 - t8743 + t8746;
    t21796
}
