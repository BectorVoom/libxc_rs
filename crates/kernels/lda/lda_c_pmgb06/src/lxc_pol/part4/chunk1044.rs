//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1044/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1044<F: Float>(t12006: F, t2088: F, t493: F, t529: F, t851: F, t5264: F, t6275: F, t1423: F, t6491: F, t6495: F, t6499: F, t1887: F, t2066: F, t9330: F, t103: F, t12325: F, t13532: F, t15345: F, t15349: F, t473: F, t9147: F, t9179: F, t9181: F, t9215: F, t9679: F, t9683: F, t9700: F, t9702: F) -> (F, F, F, F, F, F, F, F) {
    let t15516 = 4.0 / 5.0 * t493 * t12006 * t851 * t529 * t2088;
    let t15518 = 8.0 / 27.0 * t6275 * t5264;
    let t15519 = t1423 * t6491;
    let t15520 = 8.0 / 135.0 * t15519;
    let t15521 = t1423 * t6495;
    let t15522 = 16.0 / 135.0 * t15521;
    let t15523 = t1423 * t6499;
    let t15524 = 8.0 / 81.0 * t15523;
    let t15526 = 2.0 / 15.0 * t1887 * t2066;
    let t15527 = 4.0 / 405.0 * t9330;
    let t15544 = 0.017777777777777778 * t13532 + 0.03199259259259259 * t9147 - 0.015996296296296297 * t9179 - 0.010664197530864198 * t9181 + 0.07464938271604939 * t9215 + 0.047988888888888886 * t12325 - 0.04 * t103 * t473 * t15349 + 0.16 * t103 * t473 * t15345 + 0.014814814814814815 * t9679 - 0.0024691358024691358 * t9683 - 0.007407407407407408 * t9700 + 0.03950617283950617 * t9702;
    (t15516, t15518, t15520, t15522, t15524, t15526, t15527, t15544)
}
