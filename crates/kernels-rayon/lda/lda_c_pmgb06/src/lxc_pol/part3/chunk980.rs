//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 980/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk980(t11628: f64, t1147: f64, t123: f64, t2164: f64, t317: f64, t113: f64, t11583: f64, t11586: f64, t11589: f64, t11596: f64, t11601: f64, t11604: f64, t11609: f64, t11611: f64, t11615: f64, t11617: f64, t11624: f64, t1233: f64, t1316: f64, t2258: f64, t297: f64, t301: f64, t4017: f64, t4358: f64, t4360: f64, t8473: f64) -> f64 {
    let t11629 = 0.004067943812504169_f64 * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = 0.5945049527603057_f64 * t11632;
    let t11637 = 36.0_f64 * t4358 * t11583 + 18.0_f64 * t4358 * t11586 - 0.01197423401025461_f64 * t297 * t11589 * t113 * t301 - 0.03592270203076383_f64 * t11596 - t11601 - 0.01197423401025461_f64 * t11604 - t11609 - 5.4655730795145296e-05_f64 * t11611 - 4.569219094474146e-06_f64 * t11615 + 0.19513566535229734_f64 * t11617 + 9.0_f64 * t1316 * t2258 * t4017 - 0.9247854820715865_f64 * t11624 + t11629 + t11633 + 18.0_f64 * t1233 * t8473 * t4360;
    t11637
}
