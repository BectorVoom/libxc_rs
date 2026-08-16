//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 804/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk804(t1901: f64, t7485: f64, t439: f64, t2570: f64, t822: f64, t2960: f64, t2578: f64, t1385: f64, t6516: f64, t764: f64, t2871: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7547 = t1901 * t7485;
    let t7549 = t439 * t7547 / 9.0_f64;
    let t7550 = t2570 * t822;
    let t7551 = t2960 * t7550;
    let t7553 = t439 * t7551 / 9.0_f64;
    let t7554 = t2578 * t822;
    let t7555 = t1385 * t7554;
    let t7557 = t439 * t7555 / 15.0_f64;
    let t7558 = t6516 * t764;
    let t7559 = t2871 * t7558;
    let t7561 = 2.0_f64 / 15.0_f64 * t493 * t7559;
    (t7547, t7549, t7550, t7551, t7553, t7554, t7555, t7557, t7558, t7559, t7561)
}
