//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 815/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk815(t6554: f64, t822: f64, t1966: f64, t439: f64, t4837: f64, t4845: f64, t5045: f64, t5047: f64, t183: f64, t7364: f64, t5049: f64, t5052: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7695 = t6554 * t822;
    let t7696 = t1966 * t7695;
    let t7698 = t439 * t7696 / 5.0_f64;
    let t7700 = t4837 / 45.0_f64;
    let t7701 = t4845 / 45.0_f64;
    let t7702 = t5045 / 45.0_f64;
    let t7703 = t5047 / 45.0_f64;
    let t7704 = t7364 * t183;
    let t7707 = t5049 / 45.0_f64;
    let t7708 = t5052 / 45.0_f64;
    (t7695, t7696, t7698, t7700, t7701, t7702, t7703, t7704, t7707, t7708)
}
