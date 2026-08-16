//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1078(t132: f64, t435: f64, t4978: f64, t1596: f64, t1887: f64, t3292: f64, t802: f64, t9626: f64, t9628: f64, t5040: f64, t9633: f64, t12794: f64, t12798: f64, t12801: f64, t12803: f64, t12804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12807 = t132 * t435 * t4978;
    let t12808 = t12807 / 15.0_f64;
    let t12810 = t1887 * t1596 / 5.0_f64;
    let t12812 = t802 * t3292 / 5.0_f64;
    let t12813 = t9626 / 15.0_f64;
    let t12814 = 2.0_f64 / 15.0_f64 * t9628;
    let t12816 = t132 * t435 * t5040;
    let t12817 = t12816 / 15.0_f64;
    let t12818 = 2.0_f64 / 15.0_f64 * t9633;
    let t12819 = t12794 + t12798 - t12801 + t12803 + 8.0_f64 / 81.0_f64 * t12804 - t12808 + t12810 + t12812 - t12813 - t12814 - t12817 - t12818;
    (t12808, t12810, t12812, t12813, t12814, t12817, t12818, t12819)
}
