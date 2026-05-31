//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1134/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1134<F: Float>(t123: F, t317: F, t6104: F, t740: F, t10577: F, t11564: F, t11583: F, t11596: F, t11600: F, t11604: F, t11608: F, t11611: F, t11615: F, t11617: F, t11624: F, t11628: F, t11632: F, t11640: F, t1227: F, t23: F, t2449: F, t2738: F, t342: F, t346: F, t4405: F, t5583: F, t5883: F, t6007: F, t6008: F, t6012: F, t6013: F, t6018: F, t787: F) -> F {
    let t14852 = t123 * t740 * t6104 * t317;
    let t14866 = t346 * t5883 * t787 - F::cast_from(12.0_f64) * t5583 * t11564 - F::cast_from(6.0_f64) * t11640 * t6013 - F::cast_from(0.02394846802050922_f64) * t11596 - F::cast_from(0.04789693604101844_f64) * t11600 - F::cast_from(0.02394846802050922_f64) * t11604 - F::cast_from(7.28743077268604e-05_f64) * t11608 - F::cast_from(0.00010931146159029059_f64) * t11611 - F::cast_from(9.138438188948293e-06_f64) * t11615 + F::cast_from(0.3902713307045947_f64) * t11617 + F::cast_from(6.0_f64) * t4405 * t2449 - F::cast_from(1.849570964143173_f64) * t11624 + F::cast_from(0.005423925083338892_f64) * t11628 + F::cast_from(0.7926732703470741_f64) * t11632 - F::cast_from(0.10809180959278285_f64) * t14852 + F::cast_from(6.0_f64) * t1227 * t23 * t2738 + F::cast_from(24.0_f64) * t6018 * t11583 + F::cast_from(12.0_f64) * t5583 * t6007 * t6008 * t342 - F::cast_from(6.0_f64) * t5583 * t10577 * t6012;
    t14866
}
