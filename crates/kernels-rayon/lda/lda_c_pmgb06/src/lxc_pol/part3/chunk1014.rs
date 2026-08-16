//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1014/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1014(t441: f64, t4680: f64, t439: f64, t445: f64, t1972: f64, t3174: f64, t3173: f64, t4588: f64, t493: f64, t12035: f64, t12038: f64, t12040: f64, t12042: f64, t12047: f64, t12051: f64, t12055: f64, t12058: f64, t12062: f64) -> (f64, f64, f64, f64) {
    let t12063 = t441 * t4680;
    let t12066 = t439 * t12063 * t445 / 15.0_f64;
    let t12068 = 2.0_f64 / 9.0_f64 * t1972 * t3174;
    let t12071 = 2.0_f64 / 9.0_f64 * t493 * t4588 * t3173;
    let t12072 = t12035 - t12038 - t12040 + t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071;
    (t12066, t12068, t12071, t12072)
}
