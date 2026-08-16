//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 575/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk575(t3148: f64, t334: f64, t319: f64, t1011: f64, t1028: f64, t1125: f64, t3016: f64, t3081: f64, t3082: f64, t3085: f64, t3086: f64, t3095: f64, t3098: f64, t3101: f64, t3105: f64, t3112: f64, t3118: f64, t3121: f64, t3125: f64, t3133: f64, t3139: f64, t370: f64, t372: f64, t380: f64, t4: f64, t71: f64, t84: f64, t972: f64, t983: f64, t989: f64) -> (f64, f64, f64, f64) {
    let t3149 = t3148 * t334;
    let t3150 = t319 * t3149;
    let t3151 = 1.0_f64 * t3150;
    let t3152 = 1025.3897021007795_f64 * t3081 * t3082 - 103.89453539625518_f64 * t3085 * t3086 - 6.0_f64 * t972 * t372 * t983 - t3016 + 0.0016562449037037037_f64 * t4 * t1125 * t71 + 0.5848223397455204_f64 * t380 * t3095 + 3.5089340384731225_f64 * t1028 * t3098 + 96.4940495336121_f64 * t989 * t3101 * t370 - 3.5089340384731225_f64 * t1011 * t3105 + 0.0005696928233656539_f64 * t4 * t1125 * t84 + 51.94726769812759_f64 * t1028 * t3112 - t3118 + t3121 - t3125 - t3133 + t3139 - t3151;
    (t3149, t3150, t3151, t3152)
}
