//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1173/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1173(t1444: f64, t5477: f64, t1972: f64, t2988: f64, t1420: f64, t5242: f64, t439: f64, t4672: f64, t5225: f64, t12382: f64, t1897: f64, t5233: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13995 = 2.0_f64 / 15.0_f64 * t1444 * t5477;
    let t13997 = t1972 * t2988 / 15.0_f64;
    let t13999 = 2.0_f64 / 15.0_f64 * t1420 * t5242;
    let t14002 = 2.0_f64 / 15.0_f64 * t439 * t5225 * t4672;
    let t14005 = 2.0_f64 / 45.0_f64 * t439 * t1897 * t12382;
    let t14007 = 2.0_f64 / 15.0_f64 * t1420 * t5233;
    (t13995, t13997, t13999, t14002, t14005, t14007)
}
