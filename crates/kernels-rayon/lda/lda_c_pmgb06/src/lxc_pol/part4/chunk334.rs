//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 334/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk334(t113: f64, t1139: f64, t301: f64, t413: f64, t718: f64, t100: f64, t246: f64) -> (f64, f64, f64) {
    let t1141 = t1139 * t113 * t301;
    let t1145 = 0.0005811348303577384_f64 * t718 * t413 * t301;
    let t1147 = 1.0_f64 / t100 / t246;
    (t1141, t1145, t1147)
}
