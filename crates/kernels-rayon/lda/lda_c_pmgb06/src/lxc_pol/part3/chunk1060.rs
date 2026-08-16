//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1060/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1060(t12599: f64, t1981: f64, t5470: f64, t1423: f64, t5233: f64, t1825: f64, t2938: f64, t1915: f64, t493: f64, t1972: f64, t3300: f64, t2993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12602 = 16.0_f64 / 27.0_f64 * t1981 * t5470 * t12599;
    let t12603 = t1423 * t5233;
    let t12604 = 4.0_f64 / 45.0_f64 * t12603;
    let t12605 = t1825 * t2938;
    let t12608 = 2.0_f64 / 45.0_f64 * t493 * t1915 * t12605;
    let t12610 = t1972 * t3300 / 9.0_f64;
    let t12612 = t1972 * t2993 / 9.0_f64;
    (t12602, t12604, t12605, t12608, t12610, t12612)
}
