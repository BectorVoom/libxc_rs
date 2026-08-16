//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1305/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1305<F: Float>(t17152: F, t2909: F, t36: F, t15503: F, t506: F, t103: F, t1576: F, t16003: F, t17127: F, t17129: F, t17131: F, t17133: F, t17136: F, t17138: F, t17140: F, t17145: F, t17149: F) -> (F, F, F) {
    let t17154 = t36 * t2909 * t17152;
    let t17157 = t36 * t506 * t15503;
    let t17159 = -F::cast_from(0.08_f64) * t103 * t1576 * t16003 + F::cast_from(0.003950617283950617_f64) * t17127 - F::cast_from(0.03851851851851852_f64) * t17129 + F::cast_from(1.0557555555555556_f64) * t17131 - F::cast_from(0.09597777777777777_f64) * t17133 - F::cast_from(0.21595_f64) * t17136 - F::cast_from(0.047988888888888886_f64) * t17138 + F::cast_from(0.015996296296296297_f64) * t17140 - F::cast_from(0.047988888888888886_f64) * t17145 - F::cast_from(0.023994444444444443_f64) * t17149 - F::cast_from(0.03999074074074074_f64) * t17154 + F::cast_from(0.14396666666666666_f64) * t17157;
    (t17154, t17157, t17159)
}
