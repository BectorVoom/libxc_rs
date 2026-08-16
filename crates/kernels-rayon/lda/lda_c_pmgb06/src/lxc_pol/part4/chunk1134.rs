//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1134/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1134(t123: f64, t317: f64, t6104: f64, t740: f64, t10577: f64, t11564: f64, t11583: f64, t11596: f64, t11600: f64, t11604: f64, t11608: f64, t11611: f64, t11615: f64, t11617: f64, t11624: f64, t11628: f64, t11632: f64, t11640: f64, t1227: f64, t23: f64, t2449: f64, t2738: f64, t342: f64, t346: f64, t4405: f64, t5583: f64, t5883: f64, t6007: f64, t6008: f64, t6012: f64, t6013: f64, t6018: f64, t787: f64) -> f64 {
    let t14852 = t123 * t740 * t6104 * t317;
    let t14866 = t346 * t5883 * t787 - 12.0_f64 * t5583 * t11564 - 6.0_f64 * t11640 * t6013 - 0.02394846802050922_f64 * t11596 - 0.04789693604101844_f64 * t11600 - 0.02394846802050922_f64 * t11604 - 7.28743077268604e-05_f64 * t11608 - 0.00010931146159029059_f64 * t11611 - 9.138438188948293e-06_f64 * t11615 + 0.3902713307045947_f64 * t11617 + 6.0_f64 * t4405 * t2449 - 1.849570964143173_f64 * t11624 + 0.005423925083338892_f64 * t11628 + 0.7926732703470741_f64 * t11632 - 0.10809180959278285_f64 * t14852 + 6.0_f64 * t1227 * t23 * t2738 + 24.0_f64 * t6018 * t11583 + 12.0_f64 * t5583 * t6007 * t6008 * t342 - 6.0_f64 * t5583 * t10577 * t6012;
    t14866
}
