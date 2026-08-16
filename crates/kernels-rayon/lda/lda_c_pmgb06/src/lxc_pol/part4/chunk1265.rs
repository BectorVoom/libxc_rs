//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1265/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1265(t1499: f64, t2601: f64, t486: f64, t6449: f64, t1586: f64, t1992: f64, t493: f64, t6112: f64, t2002: f64, t4615: f64, t4620: f64, t1420: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16623 = t1499 * t2601 / 15.0_f64;
    let t16625 = 2.0_f64 / 15.0_f64 * t486 * t6449;
    let t16629 = t493 * t1992 * t6112 * t1586 / 15.0_f64;
    let t16631 = 2.0_f64 / 45.0_f64 * t2002 * t4615;
    let t16633 = 2.0_f64 / 27.0_f64 * t2002 * t4620;
    let t16635 = 2.0_f64 / 15.0_f64 * t1420 * t6556;
    (t16623, t16625, t16629, t16631, t16633, t16635)
}
