//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1060<F: Float>(t12041: F, t16137: F, t16144: F, t16150: F, t16152: F, t495: F, t7616: F, t493: F, t499: F, t16158: F, t16161: F, t16173: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19697 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12041;
    let t19698 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16137;
    let t19699 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t16144;
    let t19700 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t16150;
    let t19701 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t16152;
    let t19702 = t495 * t7616;
    let t19705 = t493 * t19702 * t499 / F::cast_from(45.0_f64);
    let t19706 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16158;
    let t19707 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16161;
    let t19708 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16173;
    (t19697, t19698, t19699, t19700, t19701, t19705, t19706, t19707, t19708)
}
