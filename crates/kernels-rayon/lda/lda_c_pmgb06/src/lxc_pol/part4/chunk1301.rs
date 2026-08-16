//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1301/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1301(t13384: f64, t15323: f64, t17070: f64, t13345: f64, t13347: f64, t13370: f64, t13372: f64, t13374: f64, t13376: f64, t13379: f64, t14106: f64, t14152: f64, t15548: f64, t9938: f64, t9954: f64) -> (f64, f64) {
    let t17080 = t15323 * t13384 * t17070;
    let t17097 = 0.31992592592592595_f64 * t17080 + 0.32_f64 * t15548 * t14106 * t17070 - 0.10666666666666667_f64 * t15548 * t14152 * t17070 + 0.015996296296296297_f64 * t13345 + 0.026660493827160493_f64 * t13347 + 0.12797037037037037_f64 * t13370 - 0.04265679012345679_f64 * t13372 - 0.047988888888888886_f64 * t13374 + 0.19195555555555555_f64 * t13376 - 0.09597777777777777_f64 * t13379 + 0.03950617283950617_f64 * t9938 + 0.014814814814814815_f64 * t9954;
    (t17080, t17097)
}
