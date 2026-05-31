//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 980/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk980<F: Float>(t11628: F, t1147: F, t123: F, t2164: F, t317: F, t113: F, t11583: F, t11586: F, t11589: F, t11596: F, t11601: F, t11604: F, t11609: F, t11611: F, t11615: F, t11617: F, t11624: F, t1233: F, t1316: F, t2258: F, t297: F, t301: F, t4017: F, t4358: F, t4360: F, t8473: F) -> F {
    let t11629 = F::cast_from(0.004067943812504169_f64) * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = F::cast_from(0.5945049527603057_f64) * t11632;
    let t11637 = F::cast_from(36.0_f64) * t4358 * t11583 + F::cast_from(18.0_f64) * t4358 * t11586 - F::cast_from(0.01197423401025461_f64) * t297 * t11589 * t113 * t301 - F::cast_from(0.03592270203076383_f64) * t11596 - t11601 - F::cast_from(0.01197423401025461_f64) * t11604 - t11609 - F::cast_from(5.4655730795145296e-05_f64) * t11611 - F::cast_from(4.569219094474146e-06_f64) * t11615 + F::cast_from(0.19513566535229734_f64) * t11617 + F::cast_from(9.0_f64) * t1316 * t2258 * t4017 - F::cast_from(0.9247854820715865_f64) * t11624 + t11629 + t11633 + F::cast_from(18.0_f64) * t1233 * t8473 * t4360;
    t11637
}
