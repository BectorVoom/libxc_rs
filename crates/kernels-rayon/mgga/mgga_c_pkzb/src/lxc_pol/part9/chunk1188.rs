//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1188/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1188(t2860: f64, t5578: f64, t2870: f64, t5754: f64, t1987: f64, t7223: f64, t1995: f64, t7560: f64, t1147: f64, t1306: f64, t17753: f64, t20670: f64, t20674: f64, t20676: f64, t20678: f64, t20685: f64, t2156: f64, t6062: f64, t7888: f64, t803: f64) -> (f64, f64, f64, f64, f64) {
    let t20687 = 0.5848223622634646207e0_f64 * t2860 * t5578;
    let t20693 = 0.17544670867903938621e1_f64 * t5754 * t2870;
    let t20695 = 0.51947577317044391277e2_f64 * t1987 * t7223;
    let t20697 = 0.17544670867903938621e1_f64 * t7560 * t1995;
    let t20698 = -6.0_f64 * t1147 * t1306 * t17753 * t6062 - 3.0_f64 * t1306 * t2156 * t7888 * t803 - t20670 - t20674 + t20676 + t20678 - t20685 - t20687 - t20693 - t20695 - t20697;
    (t20687, t20693, t20695, t20697, t20698)
}
