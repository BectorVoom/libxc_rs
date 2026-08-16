//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1086/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1086(t9781: f64, t1600: f64, t1992: f64, t493: f64, t4935: f64, t529: f64, t1447: f64, t5180: f64, t1972: f64, t3285: f64, t1847: f64, t607: f64) -> (f64, f64, f64, f64, f64) {
    let t12902 = 2.0_f64 / 15.0_f64 * t9781;
    let t12907 = t493 * t1992 * t1600 * t4935 * t529 / 5.0_f64;
    let t12908 = t1447 * t5180;
    let t12909 = 4.0_f64 / 15.0_f64 * t12908;
    let t12911 = t1972 * t3285 / 5.0_f64;
    let t12912 = t1847 * t607;
    (t12902, t12907, t12909, t12911, t12912)
}
