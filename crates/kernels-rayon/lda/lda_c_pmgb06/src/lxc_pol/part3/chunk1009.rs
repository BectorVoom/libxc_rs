//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1009(t1080: f64, t4865: f64, t1915: f64, t1981: f64, t2924: f64, t493: f64, t6751: f64, t1444: f64, t5487: f64, t1992: f64, t3457: f64, t1586: f64, t529: f64, t851: f64) -> (f64, f64, f64, f64, f64) {
    let t11997 = t4865 * t1080;
    let t12000 = 4.0_f64 / 5.0_f64 * t1981 * t1915 * t11997;
    let t12003 = t493 * t6751 * t2924 / 9.0_f64;
    let t12005 = 2.0_f64 / 15.0_f64 * t1444 * t5487;
    let t12006 = t1992 * t3457;
    let t12011 = 3.0_f64 / 5.0_f64 * t493 * t12006 * t851 * t1586 * t529;
    (t11997, t12000, t12003, t12005, t12011)
}
