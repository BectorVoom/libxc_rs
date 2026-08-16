//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1183/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1183(t161: f64, t489: f64, t4944: f64, t1554: f64, t2094: f64, t486: f64, t4948: f64, t10431: f64, t1385: f64, t14068: f64, t14115: f64, t14136: f64, t14160: f64, t14198: f64, t1444: f64, t1512: f64, t166: f64, t2088: f64, t2108: f64, t2885: f64, t2979: f64, t3010: f64, t3092: f64, t3441: f64, t439: f64, t493: f64, t4954: f64, t518: f64, t5276: f64, t5277: f64, t809: f64, t822: f64) -> f64 {
    let t14206 = t161 * t489 * t4944;
    let t14211 = t161 * t1554 * t2094;
    let t14212 = t14211 / 45.0_f64;
    let t14213 = t486 * t4948;
    let t14221 = -t439 * t1385 * t809 * t3441 / 45.0_f64 - 8.0_f64 / 81.0_f64 * t439 * t10431 * t822 * t3092 * t3010 - t1444 * t5277 / 15.0_f64 - t493 * t2979 * t5276 / 15.0_f64 - t14068 / 15.0_f64 - t161 * t166 * t518 * (t14115 + t14136 + t14160 + t14198) / 30.0_f64 - t14206 / 15.0_f64 - t1512 * t2108 / 10.0_f64 + t14212 - 2.0_f64 / 15.0_f64 * t14213 - t161 * t166 * t2885 * t2088 / 10.0_f64 - t486 * t4954 / 10.0_f64;
    t14221
}
