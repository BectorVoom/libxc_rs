//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1355/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1355(t4731: f64, t493: f64, t5486: f64, t1981: f64, t5441: f64, t1444: f64, t6748: f64, t176: f64, t1826: f64, t5312: f64, t13836: f64, t13838: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17819 = 2.0_f64 / 45.0_f64 * t493 * t5486 * t4731;
    let t17822 = 8.0_f64 / 45.0_f64 * t1981 * t5486 * t5441;
    let t17824 = 8.0_f64 / 45.0_f64 * t1444 * t6748;
    let t17828 = 8.0_f64 / 45.0_f64 * t493 * t5312 * t176 * t1826;
    let t17829 = 8.0_f64 / 135.0_f64 * t13836;
    let t17830 = 8.0_f64 / 27.0_f64 * t13838;
    (t17819, t17822, t17824, t17828, t17829, t17830)
}
