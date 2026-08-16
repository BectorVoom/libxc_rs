//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1450/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1450(t18542: f64, t18566: f64, t38: f64, t56: f64, t14816: f64, t64: f64, t365: f64, t5772: f64, t6996: f64, t2703: f64, t348: f64, t110: f64, t2209: f64, t30: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18568 = t18542 / 2.0_f64 + t18566 / 2.0_f64;
    let t18571 = 2.923025_f64 * t38 * t56 * t18568;
    let t18580 = 11.6921_f64 * t38 * t64 * t14816;
    let t18582 = t365 * t6996 * t5772;
    let t18585 = t348 * t2703 * t5772;
    let t18586 = 5.84605_f64 * t18585;
    let t18588 = t30 * t110 * t2209;
    (t18568, t18571, t18580, t18582, t18586, t18588)
}
