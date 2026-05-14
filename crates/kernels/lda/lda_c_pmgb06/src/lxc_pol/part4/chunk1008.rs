//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1008/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1008<F: Float>(t123: F, t317: F, t6104: F, t740: F, t10577: F, t11564: F, t11583: F, t11596: F, t11600: F, t11604: F, t11608: F, t11611: F, t11615: F, t11617: F, t11624: F, t11628: F, t11632: F, t11640: F, t1227: F, t23: F, t2449: F, t2738: F, t342: F, t346: F, t4405: F, t5583: F, t5883: F, t6007: F, t6008: F, t6012: F, t6013: F, t6018: F, t787: F) -> (F,) {
    let t14852 = t123 * t740 * t6104 * t317;
    let t14866 = t346 * t5883 * t787 - 12.0 * t5583 * t11564 - 6.0 * t11640 * t6013 - 0.02394846802050922 * t11596 - 0.04789693604101844 * t11600 - 0.02394846802050922 * t11604 - 7.28743077268604e-05 * t11608 - 0.00010931146159029059 * t11611 - 9.138438188948293e-06 * t11615 + 0.3902713307045947 * t11617 + 6.0 * t4405 * t2449 - 1.849570964143173 * t11624 + 0.005423925083338892 * t11628 + 0.7926732703470741 * t11632 - 0.10809180959278285 * t14852 + 6.0 * t1227 * t23 * t2738 + 24.0 * t6018 * t11583 + 12.0 * t5583 * t6007 * t6008 * t342 - 6.0 * t5583 * t10577 * t6012;
    (t14866,)
}
