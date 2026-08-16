//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1204/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1204<F: Float>(t273: F, t698: F, t7402: F, t11110: F, t11113: F, t11115: F, t11117: F, t11119: F, t11123: F, t11124: F, t21770: F, t248: F, t285: F, t8724: F, t8727: F, t8733: F, t8737: F, t8738: F, t8743: F, t8746: F) -> F {
    let t21787 = t7402 * t273 * t698;
    let t21796 = F::cast_from(103.89515463408878_f64) * t8724 - F::cast_from(36.0_f64) * t11110 - t11113 + t8727 - F::cast_from(0.5848223622634646_f64) * t21787 + t248 * t21770 * t285 + t8733 - F::cast_from(0.09759223170271566_f64) * t11115 - F::cast_from(0.06506148780181044_f64) * t11117 + F::cast_from(1.4447919941302971_f64) * t11119 + t11123 + F::cast_from(0.04879611585135783_f64) * t11124 - t8737 - F::cast_from(3.5089341735807875_f64) * t8738 - t8743 + t8746;
    t21796
}
