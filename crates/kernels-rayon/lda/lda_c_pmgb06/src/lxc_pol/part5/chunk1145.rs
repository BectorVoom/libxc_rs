//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1145/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1145(t1385: f64, t2064: f64, t2574: f64, t439: f64, t493: f64, t5486: f64, t6782: f64, t1420: f64, t7711: f64, t2948: f64, t7710: f64, t17563: f64) -> (f64, f64, f64, f64, f64) {
    let t20759 = 2.0_f64 / 15.0_f64 * t439 * t1385 * t2574 * t2064;
    let t20762 = t493 * t5486 * t6782 / 15.0_f64;
    let t20764 = 2.0_f64 / 15.0_f64 * t1420 * t7711;
    let t20767 = 2.0_f64 / 15.0_f64 * t439 * t2948 * t7710;
    let t20768 = t17563 / 45.0_f64;
    (t20759, t20762, t20764, t20767, t20768)
}
