//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 424/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk424(t1601: f64, t1602: f64, t166: f64, t161: f64, t1521: f64, t135: f64, t146: f64, t1568: f64, t405: f64, t474: f64, t133: f64, t134: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1603 = t1601 * t1602;
    let t1604 = t166 * t1603;
    let t1606 = t161 * t1604 / 15.0_f64;
    let t1607 = 0.047988888888888886_f64 * t1521;
    let t1614 = 0.011111111111111112_f64 * t146 * t1568 * t135;
    let t1615 = t405 * t474;
    let t1618 = 1.0_f64 / t134 / t133;
    (t1603, t1604, t1606, t1607, t1614, t1615, t1618)
}
