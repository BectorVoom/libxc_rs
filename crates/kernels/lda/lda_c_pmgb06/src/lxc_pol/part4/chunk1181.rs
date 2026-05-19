//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1181/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1181<F: Float>(t15521: F, t1423: F, t6499: F, t1887: F, t2066: F, t9330: F, t103: F, t12325: F, t13532: F, t15345: F, t15349: F, t473: F, t9147: F, t9179: F, t9181: F, t9215: F, t9679: F, t9683: F, t9700: F, t9702: F) -> (F, F, F, F, F) {
    let t15522 = F::new(16.0) / F::new(135.0) * t15521;
    let t15523 = t1423 * t6499;
    let t15524 = F::new(8.0) / F::new(81.0) * t15523;
    let t15526 = F::new(2.0) / F::new(15.0) * t1887 * t2066;
    let t15527 = F::new(4.0) / F::new(405.0) * t9330;
    let t15544 = F::cast_from(0.017777777777777778_f64) * t13532 + F::cast_from(0.03199259259259259_f64) * t9147 - F::cast_from(0.015996296296296297_f64) * t9179 - F::cast_from(0.010664197530864198_f64) * t9181 + F::cast_from(0.07464938271604939_f64) * t9215 + F::cast_from(0.047988888888888886_f64) * t12325 - F::new(0.04) * t103 * t473 * t15349 + F::new(0.16) * t103 * t473 * t15345 + F::cast_from(0.014814814814814815_f64) * t9679 - F::cast_from(0.0024691358024691358_f64) * t9683 - F::cast_from(0.007407407407407408_f64) * t9700 + F::cast_from(0.03950617283950617_f64) * t9702;
    (t15522, t15524, t15526, t15527, t15544)
}
