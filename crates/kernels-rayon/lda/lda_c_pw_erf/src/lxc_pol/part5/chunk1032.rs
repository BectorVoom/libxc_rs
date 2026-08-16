//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1032/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1032(t1325: f64, t3859: f64, t6322: f64, t3794: f64, t6292: f64, t12695: f64, t6454: f64, t1639: f64, t20: f64, t6887: f64, t1960: f64, t2123: f64) -> (f64, f64, f64, f64, f64) {
    let t17886 = t1325 * t3859 * t6322;
    let t17901 = t3794 * t6292;
    let t17906 = t1325 * t12695 * t6454;
    let t17909 = t6887 * t20 * t1639;
    let t17979 = t1960 * t2123;
    (t17886, t17901, t17906, t17909, t17979)
}
