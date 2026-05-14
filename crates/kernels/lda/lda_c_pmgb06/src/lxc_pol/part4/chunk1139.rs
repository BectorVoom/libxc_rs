//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1139/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1139<F: Float>(t13392: F, t15323: F, t17070: F, t13388: F, t13332: F, t13337: F, t17049: F, t17052: F, t17054: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F, t13384: F, t13345: F, t13347: F, t13370: F, t13372: F, t13374: F, t13376: F, t13379: F, t14106: F, t14152: F, t15548: F, t9938: F, t9954: F) -> (F, F, F, F, F) {
    let t17072 = t15323 * t13392 * t17070;
    let t17075 = t15323 * t13388 * t17070;
    let t17077 = -0.19195555555555555 * t17049 + 0.5758666666666666 * t17052 + 0.023994444444444443 * t17054 + 0.09597777777777777 * t17057 + 0.03199259259259259 * t17059 - 0.010664197530864198 * t17061 - 0.2879333333333333 * t17064 - 0.015996296296296297 * t17066 + 0.14396666666666666 * t13332 - 0.06398518518518519 * t13337 + 1.7276 * t17072 - 1.1517333333333333 * t17075;
    let t17080 = t15323 * t13384 * t17070;
    let t17097 = 0.31992592592592595 * t17080 + 0.32 * t15548 * t14106 * t17070 - 0.10666666666666667 * t15548 * t14152 * t17070 + 0.015996296296296297 * t13345 + 0.026660493827160493 * t13347 + 0.12797037037037037 * t13370 - 0.04265679012345679 * t13372 - 0.047988888888888886 * t13374 + 0.19195555555555555 * t13376 - 0.09597777777777777 * t13379 + 0.03950617283950617 * t9938 + 0.014814814814814815 * t9954;
    (t17072, t17075, t17077, t17080, t17097)
}
