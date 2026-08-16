//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 366/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk366(t1521: f64, t135: f64, t146: f64, t1568: f64, t405: f64, t474: f64, t133: f64, t134: f64, t147: f64) -> (f64, f64, f64, f64, f64) {
    let t1607 = 0.047988888888888886_f64 * t1521;
    let t1614 = 0.011111111111111112_f64 * t146 * t1568 * t135;
    let t1615 = t405 * t474;
    let t1618 = 1.0_f64 / t134 / t133;
    let t1619 = t147 * t1618;
    (t1607, t1614, t1615, t1618, t1619)
}
