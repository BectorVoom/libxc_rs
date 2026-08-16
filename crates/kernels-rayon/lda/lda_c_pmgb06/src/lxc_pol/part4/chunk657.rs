//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 657/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk657(t1623: f64, t405: f64, t1554: f64, t530: f64, t161: f64, t1587: f64, t489: f64, t516: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3428 = t405 * t1623;
    let t3450 = t1554 * t530;
    let t3451 = t161 * t3450;
    let t3453 = t489 * t1587;
    let t3454 = t161 * t3453;
    let t3456 = t516 * t516;
    let t3457 = 1.0_f64 / t3456;
    (t3428, t3450, t3451, t3453, t3454, t3456, t3457)
}
