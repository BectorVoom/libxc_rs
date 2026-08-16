//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1052/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1052(t27: f64, t34: f64, t1435: f64, t5075: f64, t5087: f64, t1438: f64, t1593: f64, t1594: f64, t332: f64, t760: f64, t5083: f64, t12501: f64, t5138: f64, t5139: f64) -> (f64, f64, f64, f64, f64) {
    let t12514 = t27 * t34;
    let t12516 = t5075 * t12514 * t1435;
    let t12517 = t12516 * t5087;
    let t12518 = 4.0_f64 / 27.0_f64 * t12517;
    let t12519 = t1593 * t1438;
    let t12521 = t760 * t1594 * t332;
    let t12524 = 2.0_f64 / 9.0_f64 * t5083 * t12519 * t12521;
    let t12527 = t5138 * t5139 * t12501 / 9.0_f64;
    (t12514, t12518, t12521, t12524, t12527)
}
