//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1060(t12041: f64, t16137: f64, t16144: f64, t16150: f64, t16152: f64, t495: f64, t7616: f64, t493: f64, t499: f64, t16158: f64, t16161: f64, t16173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19697 = 8.0_f64 / 405.0_f64 * t12041;
    let t19698 = 2.0_f64 / 15.0_f64 * t16137;
    let t19699 = 4.0_f64 / 135.0_f64 * t16144;
    let t19700 = 2.0_f64 / 81.0_f64 * t16150;
    let t19701 = 2.0_f64 / 135.0_f64 * t16152;
    let t19702 = t495 * t7616;
    let t19705 = t493 * t19702 * t499 / 45.0_f64;
    let t19706 = 4.0_f64 / 45.0_f64 * t16158;
    let t19707 = 2.0_f64 / 45.0_f64 * t16161;
    let t19708 = 2.0_f64 / 45.0_f64 * t16173;
    (t19697, t19698, t19699, t19700, t19701, t19705, t19706, t19707, t19708)
}
