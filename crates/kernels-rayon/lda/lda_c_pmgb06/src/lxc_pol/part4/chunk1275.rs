//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1275/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1275(t131: f64, t155: f64, t16769: f64, t44: f64, t460: f64, t6705: f64, t1592: f64, t6225: f64, t1966: f64, t439: f64, t477: f64, t9828: f64) -> (f64, f64, f64, f64) {
    let t16773 = t16769 * t44 * t131 * t155 / 30.0_f64;
    let t16775 = t6705 * t460 / 15.0_f64;
    let t16776 = t1592 * t6225;
    let t16780 = 2.0_f64 / 15.0_f64 * t439 * t1966 * t16776 * t477;
    let t16781 = 4.0_f64 / 405.0_f64 * t9828;
    (t16773, t16775, t16780, t16781)
}
