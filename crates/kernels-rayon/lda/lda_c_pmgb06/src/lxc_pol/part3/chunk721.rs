//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 721/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk721(t44: f64, t4752: f64, t131: f64, t155: f64, t1416: f64, t1988: f64, t493: f64, t1602: f64, t3457: f64, t851: f64, t1992: f64, t1594: f64, t3031: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4753 = t4752 * t44;
    let t4754 = t4753 * t131;
    let t4756 = t4754 * t155 / 30.0_f64;
    let t4757 = t1988 * t1416;
    let t4759 = 2.0_f64 / 45.0_f64 * t493 * t4757;
    let t4761 = t3457 * t851 * t1602;
    let t4762 = t1992 * t4761;
    let t4764 = t493 * t4762 / 5.0_f64;
    let t4766 = t3031 * t822 * t1594;
    (t4753, t4754, t4756, t4757, t4759, t4761, t4762, t4764, t4766)
}
