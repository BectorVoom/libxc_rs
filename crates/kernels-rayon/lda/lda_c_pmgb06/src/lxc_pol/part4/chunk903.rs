//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 903/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk903(t493: f64, t6545: f64, t1969: f64, t2002: f64, t136: f64, t813: f64, t1968: f64, t439: f64, t1592: f64, t2648: f64, t477: f64, t1966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6547 = t493 * t6545 / 45.0_f64;
    let t6549 = 2.0_f64 / 15.0_f64 * t2002 * t1969;
    let t6550 = t136 * t813;
    let t6551 = t6550 * t1968;
    let t6553 = 2.0_f64 / 15.0_f64 * t439 * t6551;
    let t6554 = t1592 * t2648;
    let t6555 = t6554 * t477;
    let t6556 = t1966 * t6555;
    (t6547, t6549, t6550, t6551, t6553, t6554, t6555, t6556)
}
