//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1103/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1103(t1915: f64, t19336: f64, t1981: f64, t1385: f64, t439: f64, t477: f64, t7489: f64, t1897: f64, t19766: f64, t1901: f64, t19786: f64, t16612: f64) -> (f64, f64, f64, f64, f64) {
    let t20264 = 4.0_f64 / 5.0_f64 * t1981 * t1915 * t19336;
    let t20268 = 2.0_f64 / 15.0_f64 * t439 * t1385 * t7489 * t477;
    let t20271 = 8.0_f64 / 15.0_f64 * t439 * t1897 * t19766;
    let t20274 = 4.0_f64 / 3.0_f64 * t439 * t1901 * t19786;
    let t20275 = 2.0_f64 / 15.0_f64 * t16612;
    (t20264, t20268, t20271, t20274, t20275)
}
