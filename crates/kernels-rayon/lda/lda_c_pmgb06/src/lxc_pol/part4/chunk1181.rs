//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1181/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1181(t15521: f64, t1423: f64, t6499: f64, t1887: f64, t2066: f64, t9330: f64, t103: f64, t12325: f64, t13532: f64, t15345: f64, t15349: f64, t473: f64, t9147: f64, t9179: f64, t9181: f64, t9215: f64, t9679: f64, t9683: f64, t9700: f64, t9702: f64) -> (f64, f64, f64, f64, f64) {
    let t15522 = 16.0_f64 / 135.0_f64 * t15521;
    let t15523 = t1423 * t6499;
    let t15524 = 8.0_f64 / 81.0_f64 * t15523;
    let t15526 = 2.0_f64 / 15.0_f64 * t1887 * t2066;
    let t15527 = 4.0_f64 / 405.0_f64 * t9330;
    let t15544 = 0.017777777777777778_f64 * t13532 + 0.03199259259259259_f64 * t9147 - 0.015996296296296297_f64 * t9179 - 0.010664197530864198_f64 * t9181 + 0.07464938271604939_f64 * t9215 + 0.047988888888888886_f64 * t12325 - 0.04_f64 * t103 * t473 * t15349 + 0.16_f64 * t103 * t473 * t15345 + 0.014814814814814815_f64 * t9679 - 0.0024691358024691358_f64 * t9683 - 0.007407407407407408_f64 * t9700 + 0.03950617283950617_f64 * t9702;
    (t15522, t15524, t15526, t15527, t15544)
}
