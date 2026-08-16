//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1305/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1305(t17152: f64, t2909: f64, t36: f64, t15503: f64, t506: f64, t103: f64, t1576: f64, t16003: f64, t17127: f64, t17129: f64, t17131: f64, t17133: f64, t17136: f64, t17138: f64, t17140: f64, t17145: f64, t17149: f64) -> (f64, f64, f64) {
    let t17154 = t36 * t2909 * t17152;
    let t17157 = t36 * t506 * t15503;
    let t17159 = -0.08_f64 * t103 * t1576 * t16003 + 0.003950617283950617_f64 * t17127 - 0.03851851851851852_f64 * t17129 + 1.0557555555555556_f64 * t17131 - 0.09597777777777777_f64 * t17133 - 0.21595_f64 * t17136 - 0.047988888888888886_f64 * t17138 + 0.015996296296296297_f64 * t17140 - 0.047988888888888886_f64 * t17145 - 0.023994444444444443_f64 * t17149 - 0.03999074074074074_f64 * t17154 + 0.14396666666666666_f64 * t17157;
    (t17154, t17157, t17159)
}
