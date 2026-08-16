//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 870/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk870(t500: f64, t6134: f64, t1972: f64, t1989: f64, t2855: f64, t2858: f64, t2854: f64, t4593: f64, t4624: f64, t4717: f64, t6111: f64, t6116: f64, t6118: f64, t6122: f64, t6126: f64, t6129: f64, t6133: f64) -> (f64, f64, f64, f64, f64) {
    let t6136 = t6134 * t500 / 45.0_f64;
    let t6138 = 2.0_f64 / 45.0_f64 * t1972 * t1989;
    let t6139 = t2855 / 135.0_f64;
    let t6140 = t2858 / 135.0_f64;
    let t6141 = t6111 + t6116 + t6118 + t6122 + t6126 + t6129 + t6133 + t6136 + t6138 + t2854 - t6139 - t6140 + t4593 + t4624 - t4717;
    (t6136, t6138, t6139, t6140, t6141)
}
