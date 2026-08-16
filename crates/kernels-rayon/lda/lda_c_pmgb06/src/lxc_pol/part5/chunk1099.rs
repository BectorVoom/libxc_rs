//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1099/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1099(t9762: f64, t9765: f64, t16593: f64, t6616: f64, t831: f64, t486: f64, t7726: f64, t12840: f64, t161: f64, t166: f64, t2599: f64, t6232: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20209 = 4.0_f64 / 405.0_f64 * t9762;
    let t20210 = 4.0_f64 / 405.0_f64 * t9765;
    let t20211 = t16593 / 45.0_f64;
    let t20212 = t831 * t6616;
    let t20213 = t20212 / 15.0_f64;
    let t20215 = t486 * t7726 / 5.0_f64;
    let t20219 = t161 * t166 * t12840 * t2599 / 5.0_f64;
    let t20221 = t831 * t6232 / 10.0_f64;
    (t20209, t20210, t20211, t20213, t20215, t20219, t20221)
}
