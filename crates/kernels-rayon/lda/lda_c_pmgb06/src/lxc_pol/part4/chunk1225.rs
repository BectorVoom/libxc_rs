//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1225/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1225(t16137: f64, t12084: f64, t12105: f64, t12107: f64, t16112: f64, t16114: f64, t16117: f64, t16121: f64, t16122: f64, t16124: f64, t16126: f64, t16130: f64, t16132: f64, t16135: f64, t16136: f64) -> (f64, f64, f64, f64, f64) {
    let t16138 = 4.0_f64 / 45.0_f64 * t16137;
    let t16139 = 8.0_f64 / 45.0_f64 * t12084;
    let t16140 = 4.0_f64 / 45.0_f64 * t12105;
    let t16141 = 2.0_f64 / 45.0_f64 * t12107;
    let t16142 = t16112 + t16114 + t16117 + t16121 + t16122 + t16124 + t16126 + t16130 - t16132 + t16135 - t16136 + t16138 + t16139 + t16140 + t16141;
    (t16138, t16139, t16140, t16141, t16142)
}
