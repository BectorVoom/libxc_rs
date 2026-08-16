//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1026/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1026(t1512: f64, t2066: f64, t2043: f64, t1447: f64, t5282: f64, t2912: f64, t2918: f64, t2991: f64, t493: f64, t851: f64, t1444: f64, t5337: f64) -> (f64, f64, f64, f64, f64) {
    let t12199 = t1512 * t2066 / 10.0_f64;
    let t12201 = t1512 * t2043 / 10.0_f64;
    let t12202 = t1447 * t5282;
    let t12203 = 2.0_f64 / 27.0_f64 * t12202;
    let t12208 = 2.0_f64 / 9.0_f64 * t493 * t2991 * t851 * t2918 * t2912;
    let t12210 = 2.0_f64 / 15.0_f64 * t1444 * t5337;
    (t12199, t12201, t12203, t12208, t12210)
}
