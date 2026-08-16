//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1296/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1296(t13211: f64, t13213: f64, t13215: f64, t13218: f64, t4612: f64, t6275: f64, t13220: f64, t2477: f64, t3177: f64, t2614: f64, t955: f64, t2617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17012 = 4.0_f64 / 135.0_f64 * t13211;
    let t17013 = 8.0_f64 / 135.0_f64 * t13213;
    let t17014 = 4.0_f64 / 135.0_f64 * t13215;
    let t17015 = 16.0_f64 / 135.0_f64 * t13218;
    let t17017 = 8.0_f64 / 45.0_f64 * t6275 * t4612;
    let t17018 = 4.0_f64 / 135.0_f64 * t13220;
    let t17020 = 2.0_f64 / 45.0_f64 * t3177 * t2477;
    let t17025 = t955 * t2614;
    let t17030 = t955 * t2617;
    (t17012, t17013, t17014, t17015, t17017, t17018, t17020, t17025, t17030)
}
