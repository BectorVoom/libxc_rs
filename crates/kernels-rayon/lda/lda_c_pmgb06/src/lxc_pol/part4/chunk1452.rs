//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1452/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1452(t110: f64, t360: f64, t7031: f64, t2707: f64, t348: f64, t5772: f64, t7035: f64, t7027: f64, t365: f64, t6989: f64, t18588: f64, t5770: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18609 = t360 * t110 * t7031;
    let t18615 = t348 * t2707 * t5772;
    let t18616 = 1.9486833333333333_f64 * t18615;
    let t18622 = t360 * t110 * t7035;
    let t18625 = t360 * t110 * t7027;
    let t18628 = t365 * t6989 * t5772;
    let t18630 = t5770 * t18588;
    (t18609, t18616, t18622, t18625, t18628, t18630)
}
