//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 615/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk615<F: Float>(t3138: F, t1953: F, t2061: F, t2717: F, t2720: F, t2723: F, t2728: F, t2730: F, t2732: F, t334: F, t319: F, t1011: F, t1028: F, t1125: F, t3016: F, t3081: F, t3082: F, t3085: F, t3086: F, t3095: F, t3098: F, t3101: F, t3105: F, t3112: F, t3118: F, t3121: F, t3125: F, t3133: F, t370: F, t372: F, t380: F, t4: F, t71: F, t84: F, t972: F, t983: F, t989: F) -> (F, F, F, F, F, F) {
    let t3139 = 96.49094593290663 * t3138;
    let t3148 = -2.5319 * t2717 + 1.6879333333333333 * t2720 - 1.9692555555555555 * t2723 - 0.9301185185185186 * t1953 + 0.13651666666666668 * t2728 - 0.27303333333333335 * t2730 - 0.31853888888888887 * t2732 - 0.36514074074074077 * t2061;
    let t3149 = t3148 * t334;
    let t3150 = t319 * t3149;
    let t3151 = 1.0 * t3150;
    let t3152 = 1025.3897021007795 * t3081 * t3082 - 103.89453539625518 * t3085 * t3086 - 6.0 * t972 * t372 * t983 - t3016 + 0.0016562449037037037 * t4 * t1125 * t71 + 0.5848223397455204 * t380 * t3095 + 3.5089340384731225 * t1028 * t3098 + 96.4940495336121 * t989 * t3101 * t370 - 3.5089340384731225 * t1011 * t3105 + 0.0005696928233656539 * t4 * t1125 * t84 + 51.94726769812759 * t1028 * t3112 - t3118 + t3121 - t3125 - t3133 + t3139 - t3151;
    (t3139, t3148, t3149, t3150, t3151, t3152)
}
