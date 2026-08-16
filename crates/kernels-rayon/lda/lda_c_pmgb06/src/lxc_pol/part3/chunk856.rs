//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 856/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk856(t1092: f64, t1105: f64, t1065: f64, t1089: f64, t248: f64, t2799: f64, t687: f64, t3767: f64, t638: f64, t1090: f64, t1101: f64, t643: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8529 = t1105 * t1092;
    let t8534 = t248 * t1089 * t1065;
    let t8536 = t2799 * t687;
    let t8538 = t638 * t3767;
    let t8541 = t1105 * t1090;
    let t8543 = t1101 * t1092;
    let t8545 = t643 * t3767;
    (t8529, t8534, t8536, t8538, t8541, t8543, t8545)
}
