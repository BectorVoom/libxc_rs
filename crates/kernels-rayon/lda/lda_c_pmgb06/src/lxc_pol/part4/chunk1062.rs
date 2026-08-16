//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1062/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1062(t1183: f64, t1798: f64, t297: f64, t301: f64, t2789: f64, t794: f64, t1767: f64, t1770: f64, t419: f64, t1186: f64, t5899: f64, t1193: f64, t4001: f64, t4299: f64) -> (f64, f64, f64, f64, f64) {
    let t11600 = t297 * t1798 * t1183 * t301;
    let t11604 = t297 * t794 * t2789 * t301;
    let t11608 = t1767 * t1798 * t419 * t1770;
    let t11611 = t5899 * t1186 * t1770;
    let t11615 = t4001 * t794 * t1193 * t4299;
    (t11600, t11604, t11608, t11611, t11615)
}
