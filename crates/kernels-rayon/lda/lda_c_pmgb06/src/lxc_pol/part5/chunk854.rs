//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 854/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk854(t957: f64, t682: f64, t696: f64, t978: f64, t1066: f64, t1108: f64, t1092: f64, t1105: f64, t2799: f64, t687: f64, t3767: f64, t638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8522 = t957 * t957;
    let t8526 = 3.5089341735807875_f64 * t696 * t978 * t8522 * t682;
    let t8527 = t1108 * t1066;
    let t8529 = t1105 * t1092;
    let t8536 = t2799 * t687;
    let t8538 = t638 * t3767;
    (t8522, t8526, t8527, t8529, t8536, t8538)
}
