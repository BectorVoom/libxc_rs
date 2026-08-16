//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 990/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk990(t9110: f64, t9234: f64, t3005: f64, t831: f64, t9237: f64, t9239: f64, t9242: f64, t9259: f64, t9267: f64, t9269: f64, t9272: f64, t9274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11775 = 16.0_f64 / 243.0_f64 * t9110;
    let t11776 = t9234 / 45.0_f64;
    let t11777 = t831 * t3005;
    let t11778 = 4.0_f64 / 405.0_f64 * t11777;
    let t11779 = t9237 / 45.0_f64;
    let t11780 = t9239 / 45.0_f64;
    let t11781 = t9242 / 45.0_f64;
    let t11782 = t9259 / 45.0_f64;
    let t11783 = 4.0_f64 / 135.0_f64 * t9267;
    let t11784 = 4.0_f64 / 135.0_f64 * t9269;
    let t11785 = 2.0_f64 / 45.0_f64 * t9272;
    let t11786 = 4.0_f64 / 45.0_f64 * t9274;
    (t11775, t11776, t11778, t11779, t11780, t11781, t11782, t11783, t11784, t11785, t11786)
}
