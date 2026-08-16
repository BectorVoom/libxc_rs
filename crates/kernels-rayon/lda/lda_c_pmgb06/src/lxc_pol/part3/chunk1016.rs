//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1016/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1016(t1430: f64, t439: f64, t4779: f64, t1435: f64, t1872: f64, t1440: f64, t2002: f64, t3217: f64, t3276: f64, t3280: f64, t1420: f64, t4780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12091 = t439 * t4779 * t1430 / 15.0_f64;
    let t12092 = t1435 * t1872;
    let t12095 = t439 * t12092 * t1440 / 9.0_f64;
    let t12097 = t2002 * t3217 / 15.0_f64;
    let t12099 = t2002 * t3276 / 15.0_f64;
    let t12101 = t2002 * t3280 / 9.0_f64;
    let t12103 = 2.0_f64 / 15.0_f64 * t1420 * t4780;
    (t12091, t12095, t12097, t12099, t12101, t12103)
}
