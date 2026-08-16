//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1092/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1092(t6227: f64, t802: f64, t16542: f64, t16549: f64, t20127: f64, t20129: f64, t20131: f64, t20133: f64, t20135: f64, t20138: f64, t20139: f64, t20140: f64) -> (f64, f64, f64, f64) {
    let t20142 = t802 * t6227 / 10.0_f64;
    let t20143 = 4.0_f64 / 27.0_f64 * t16542;
    let t20144 = 8.0_f64 / 45.0_f64 * t16549;
    let t20145 = -t20127 - t20129 + t20131 + t20133 + t20135 + t20138 - t20139 - t20140 - t20142 - t20143 + t20144;
    (t20142, t20143, t20144, t20145)
}
