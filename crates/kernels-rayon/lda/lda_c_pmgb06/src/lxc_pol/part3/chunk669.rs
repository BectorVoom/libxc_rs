//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 669/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk669(t4131: f64, t4140: f64, t44: f64, t1727: f64, t607: f64, t1710: f64, t604: f64, t1980: f64, t223: f64, t224: f64, t3009: f64, t3014: f64, t3037: f64, t3042: f64, t3045: f64, t3049: f64, t3052: f64, t3054: f64, t3057: f64, t3062: f64, t3065: f64, t3067: f64, t3070: f64) -> (f64, f64, f64, f64, f64) {
    let t4143 = (t4131 / 2.0_f64 + t4140 / 2.0_f64) * t44;
    let t4146 = t1727 * t607;
    let t4148 = t604 * t1710;
    let t4151 = 8.0_f64 / 405.0_f64 * t223 * t1980;
    let t4152 = -t3009 + t3014 - t4143 * t224 / 15.0_f64 - 2.0_f64 / 15.0_f64 * t4146 + 2.0_f64 / 45.0_f64 * t4148 - t4151 - t3037 + t3042 + t3045 - t3049 - t3052 - t3054 + t3057 - t3062 - t3065 - t3067 - t3070;
    (t4143, t4146, t4148, t4151, t4152)
}
