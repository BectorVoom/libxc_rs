//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 407/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk407(t132: f64, t1517: f64, t134: f64, t138: f64, t1470: f64, t350: f64, t455: f64, t139: f64, t441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1518 = t132 * t1517;
    let t1519 = 2.0_f64 / 45.0_f64 * t1518;
    let t1521 = t138 * t1470 * t134;
    let t1522 = 0.002518888888888889_f64 * t1521;
    let t1523 = t350 * t455;
    let t1525 = t139 * t441;
    (t1518, t1519, t1521, t1522, t1523, t1525)
}
