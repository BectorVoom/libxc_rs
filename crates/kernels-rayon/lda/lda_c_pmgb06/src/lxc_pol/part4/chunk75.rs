//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 75/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk75(t101: f64, t147: f64, t135: f64, t142: f64, t146: f64) -> (f64, f64, f64) {
    let t148 = t101 * t147;
    let t152 = 1.0_f64 + 0.107975_f64 * t142 + 0.01_f64 * t146 * t148 * t135;
    let t153 = 1.0_f64 / t152;
    (t148, t152, t153)
}
