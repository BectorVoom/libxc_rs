//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1082/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1082(t1447: f64, t7521: f64, t1919: f64, t19362: f64, t1981: f64, t1444: f64, t10293: f64, t493: f64, t7520: f64, t2088: f64, t2541: f64, t2991: f64) -> (f64, f64, f64, f64, f64) {
    let t20008 = t1447 * t7521;
    let t20009 = 2.0_f64 / 27.0_f64 * t20008;
    let t20012 = 2.0_f64 / 9.0_f64 * t1981 * t1919 * t19362;
    let t20014 = t1444 * t7521 / 9.0_f64;
    let t20017 = t493 * t10293 * t7520 / 9.0_f64;
    let t20021 = t493 * t2991 * t2541 * t2088 / 9.0_f64;
    (t20009, t20012, t20014, t20017, t20021)
}
