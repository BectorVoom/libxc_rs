//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1112/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1112(t493: f64, t6119: f64, t6527: f64, t1420: f64, t7690: f64, t18016: f64, t439: f64, t805: f64, t2477: f64, t5187: f64, t2002: f64, t6300: f64) -> (f64, f64, f64, f64, f64) {
    let t20367 = 2.0_f64 / 5.0_f64 * t493 * t6119 * t6527;
    let t20369 = t1420 * t7690 / 15.0_f64;
    let t20372 = t439 * t18016 * t805 / 15.0_f64;
    let t20374 = 2.0_f64 / 15.0_f64 * t5187 * t2477;
    let t20376 = 2.0_f64 / 15.0_f64 * t2002 * t6300;
    (t20367, t20369, t20372, t20374, t20376)
}
