//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1301/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1301<F: Float>(t13384: F, t15323: F, t17070: F, t13345: F, t13347: F, t13370: F, t13372: F, t13374: F, t13376: F, t13379: F, t14106: F, t14152: F, t15548: F, t9938: F, t9954: F) -> (F, F) {
    let t17080 = t15323 * t13384 * t17070;
    let t17097 = F::cast_from(0.31992592592592595_f64) * t17080 + F::cast_from(0.32_f64) * t15548 * t14106 * t17070 - F::cast_from(0.10666666666666667_f64) * t15548 * t14152 * t17070 + F::cast_from(0.015996296296296297_f64) * t13345 + F::cast_from(0.026660493827160493_f64) * t13347 + F::cast_from(0.12797037037037037_f64) * t13370 - F::cast_from(0.04265679012345679_f64) * t13372 - F::cast_from(0.047988888888888886_f64) * t13374 + F::cast_from(0.19195555555555555_f64) * t13376 - F::cast_from(0.09597777777777777_f64) * t13379 + F::cast_from(0.03950617283950617_f64) * t9938 + F::cast_from(0.014814814814814815_f64) * t9954;
    (t17080, t17097)
}
