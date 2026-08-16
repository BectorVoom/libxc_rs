//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 899/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk899(t175: f64, t3456: f64, t132: f64, t3034: f64, t435: f64, t152: f64, t3030: f64, t1623: f64, t955: f64, t3415: f64, t405: f64, t1620: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9636 = 1.0_f64 / t3456 / t175;
    let t9644 = t132 * t435 * t3034;
    let t9647 = 1.0_f64 / t3030 / t152;
    let t9679 = t955 * t1623;
    let t9681 = t405 * t3415;
    let t9683 = t955 * t1620;
    (t9636, t9644, t9647, t9679, t9681, t9683)
}
