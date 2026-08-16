//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1061/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1061(t16178: f64, t12113: f64, t19696: f64, t19697: f64, t19698: f64, t19699: f64, t19700: f64, t19701: f64, t19705: f64, t19706: f64, t19707: f64, t19708: f64) -> (f64, f64) {
    let t19709 = 2.0_f64 / 45.0_f64 * t16178;
    let t19710 = -t19696 + t19697 + t19698 - t19699 - t19700 - t19701 + t19705 - t19706 - t19707 + t12113 + t19708 - t19709;
    (t19709, t19710)
}
