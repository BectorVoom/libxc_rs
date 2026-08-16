//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1131/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1131(t493: f64, t6503: f64, t6751: f64, t1981: f64, t6406: f64, t6747: f64, t1444: f64, t7509: f64, t2979: f64, t7508: f64, t1380: f64, t6827: f64, t851: f64) -> (f64, f64, f64, f64, f64) {
    let t20584 = 2.0_f64 / 3.0_f64 * t493 * t6751 * t6503;
    let t20587 = 8.0_f64 / 15.0_f64 * t1981 * t6747 * t6406;
    let t20589 = t1444 * t7509 / 15.0_f64;
    let t20592 = t493 * t2979 * t7508 / 15.0_f64;
    let t20596 = t493 * t1380 * t6827 * t851 / 15.0_f64;
    (t20584, t20587, t20589, t20592, t20596)
}
