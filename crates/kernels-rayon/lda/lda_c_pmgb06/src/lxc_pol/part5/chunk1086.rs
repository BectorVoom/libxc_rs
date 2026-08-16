//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1086/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1086(t1420: f64, t7667: f64, t1426: f64, t439: f64, t7666: f64, t1444: f64, t7671: f64, t1962: f64, t6244: f64, t7577: f64, t12092: f64, t2484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20071 = 2.0_f64 / 15.0_f64 * t1420 * t7667;
    let t20074 = 2.0_f64 / 15.0_f64 * t439 * t1426 * t7666;
    let t20076 = 2.0_f64 / 15.0_f64 * t1444 * t7671;
    let t20079 = t439 * t1962 * t6244 / 15.0_f64;
    let t20081 = t1420 * t7577 / 9.0_f64;
    let t20084 = t439 * t12092 * t2484 / 9.0_f64;
    (t20071, t20074, t20076, t20079, t20081, t20084)
}
