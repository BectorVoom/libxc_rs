//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1270/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1270(t1423: f64, t6465: f64, t6475: f64, t2477: f64, t3220: f64, t6300: f64, t5211: f64, t6303: f64, t12898: f64, t1420: f64, t1444: f64, t1629: f64, t1848: f64, t1966: f64, t1967: f64, t2010: f64, t2011: f64, t2090: f64, t2481: f64, t3177: f64, t439: f64, t4779: f64, t5039: f64, t5168: f64, t6114: f64, t6241: f64, t6253: f64, t9774: f64) -> f64 {
    let t16687 = t1423 * t6465;
    let t16689 = t1423 * t6475;
    let t16697 = t3220 * t2477;
    let t16699 = t1423 * t6300;
    let t16701 = t5211 * t6303;
    let t16720 = -4.0_f64 / 81.0_f64 * t16687 + 32.0_f64 / 243.0_f64 * t16689 - 2.0_f64 / 15.0_f64 * t1848 * t2090 + t9774 / 135.0_f64 - 8.0_f64 / 135.0_f64 * t12898 + 2.0_f64 / 15.0_f64 * t1444 * t6114 + 8.0_f64 / 135.0_f64 * t16697 + 8.0_f64 / 135.0_f64 * t16699 - 4.0_f64 / 27.0_f64 * t16701 + 8.0_f64 / 45.0_f64 * t2010 * t4779 * t2011 + 8.0_f64 / 45.0_f64 * t5168 * t6303 + t3177 * t2481 / 45.0_f64 + 2.0_f64 / 45.0_f64 * t1420 * t6241 + 2.0_f64 / 15.0_f64 * t439 * t1966 * t1967 * t5039 - t439 * t1966 * t6253 * t1629 / 5.0_f64;
    t16720
}
