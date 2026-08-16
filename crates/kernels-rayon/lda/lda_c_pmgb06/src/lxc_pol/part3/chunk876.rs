//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 876/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk876(t242: f64, t2786: f64, t30: f64, t1041: f64, t1043: f64, t3697: f64, t632: f64, t2799: f64, t654: f64, t2801: f64, t3891: f64, t638: f64) -> (f64, f64, f64, f64, f64) {
    let t8837 = 0.011483599538271605_f64 * t30 * t2786 * t242;
    let t8841 = 64.32791799477015_f64 * t1041 * t3697 * t1043 * t632;
    let t8844 = t2799 * t654;
    let t8846 = t2801 * t654;
    let t8850 = t638 * t3891;
    (t8837, t8841, t8844, t8846, t8850)
}
