//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1041/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1041(t1915: f64, t19340: f64, t493: f64, t19344: f64, t1981: f64, t1444: f64, t7517: f64, t5463: f64, t7516: f64, t1919: f64, t19358: f64, t332: f64, t7284: f64, t9190: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19458 = 2.0_f64 / 15.0_f64 * t493 * t1915 * t19340;
    let t19461 = 4.0_f64 / 15.0_f64 * t1981 * t1915 * t19344;
    let t19463 = t1444 * t7517 / 9.0_f64;
    let t19466 = t493 * t5463 * t7516 / 9.0_f64;
    let t19469 = t493 * t1919 * t19358 / 9.0_f64;
    let t19471 = t9190 * t7284 * t332;
    (t19458, t19461, t19463, t19466, t19469, t19471)
}
