//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 873/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk873<F: Float>(t4320: F, t909: F, t123: F, t317: F, t4001: F, t902: F, t113: F, t1798: F, t247: F, t301: F, t1147: F, t2164: F, t11583: F, t11586: F, t11589: F, t11596: F, t11601: F, t11604: F, t11609: F, t11611: F, t11615: F, t1233: F, t1316: F, t2258: F, t297: F, t4017: F, t4358: F, t4360: F, t8473: F) -> (F,) {
    let t11617 = t4320 * t909;
    let t11624 = t123 * t4001 * t902 * t317;
    let t11628 = t247 * t1798 * t113 * t301;
    let t11629 = 0.004067943812504169 * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = 0.5945049527603057 * t11632;
    let t11637 = 36.0 * t4358 * t11583 + 18.0 * t4358 * t11586 - 0.01197423401025461 * t297 * t11589 * t113 * t301 - 0.03592270203076383 * t11596 - t11601 - 0.01197423401025461 * t11604 - t11609 - 5.4655730795145296e-05 * t11611 - 4.569219094474146e-06 * t11615 + 0.19513566535229734 * t11617 + 9.0 * t1316 * t2258 * t4017 - 0.9247854820715865 * t11624 + t11629 + t11633 + 18.0 * t1233 * t8473 * t4360;
    (t11637,)
}
