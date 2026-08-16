//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 779/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk779(t1994: f64, t5179: f64, t493: f64, t1444: f64, t1995: f64, t1447: f64, t1989: f64, t1886: f64, t224: f64) -> (f64, f64, f64, f64, f64) {
    let t5180 = t5179 * t1994;
    let t5182 = 2.0_f64 / 15.0_f64 * t493 * t5180;
    let t5184 = 2.0_f64 / 15.0_f64 * t1444 * t1995;
    let t5186 = 4.0_f64 / 135.0_f64 * t1447 * t1989;
    let t5187 = t1886 * t224;
    (t5180, t5182, t5184, t5186, t5187)
}
