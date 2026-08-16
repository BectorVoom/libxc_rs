//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1094/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1094(t2018: f64, t2563: f64, t16558: f64, t439: f64, t5482: f64, t6412: f64, t6160: f64, t6494: f64, t6165: f64, t6498: f64, t13933: f64, t6464: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20160 = t2563 * t2018;
    let t20161 = t20160 / 15.0_f64;
    let t20162 = t16558 / 15.0_f64;
    let t20165 = t439 * t5482 * t6412 / 15.0_f64;
    let t20168 = 2.0_f64 / 15.0_f64 * t439 * t6494 * t6160;
    let t20171 = t439 * t6498 * t6165 / 9.0_f64;
    let t20174 = t439 * t13933 * t6464 / 9.0_f64;
    (t20161, t20162, t20165, t20168, t20171, t20174)
}
