//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1136/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1136(t1444: f64, t7660: f64, t3238: f64, t493: f64, t7659: f64, t7567: f64, t10220: f64, t7566: f64, t439: f64, t6550: f64, t6555: f64, t2481: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20643 = 2.0_f64 / 9.0_f64 * t1444 * t7660;
    let t20646 = 2.0_f64 / 9.0_f64 * t493 * t3238 * t7659;
    let t20648 = 8.0_f64 / 81.0_f64 * t1444 * t7567;
    let t20651 = 8.0_f64 / 81.0_f64 * t493 * t10220 * t7566;
    let t20654 = t439 * t6550 * t6555 / 5.0_f64;
    let t20656 = t5187 * t2481 / 15.0_f64;
    (t20643, t20646, t20648, t20651, t20654, t20656)
}
