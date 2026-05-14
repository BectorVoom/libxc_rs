//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1142/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1142<F: Float>(t1476: F, t17143: F, t36: F, t1083: F, t6764: F, t1080: F, t2389: F, t2911: F, t2909: F, t15503: F, t506: F, t103: F, t1576: F, t16003: F, t17127: F, t17129: F, t17131: F, t17133: F, t17136: F, t17138: F, t17140: F) -> (F, F, F, F, F, F, F) {
    let t17145 = t36 * t1476 * t17143;
    let t17147 = t6764 * t1083;
    let t17149 = t36 * t1476 * t17147;
    let t17152 = t2911 * t2389 * t1080;
    let t17154 = t36 * t2909 * t17152;
    let t17157 = t36 * t506 * t15503;
    let t17159 = -0.08 * t103 * t1576 * t16003 + 0.003950617283950617 * t17127 - 0.03851851851851852 * t17129 + 1.0557555555555556 * t17131 - 0.09597777777777777 * t17133 - 0.21595 * t17136 - 0.047988888888888886 * t17138 + 0.015996296296296297 * t17140 - 0.047988888888888886 * t17145 - 0.023994444444444443 * t17149 - 0.03999074074074074 * t17154 + 0.14396666666666666 * t17157;
    (t17145, t17147, t17149, t17152, t17154, t17157, t17159)
}
