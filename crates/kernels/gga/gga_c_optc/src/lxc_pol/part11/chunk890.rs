//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 890/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk890<F: Float>(t1102: F, t17502: F, t4300: F, t5110: F, t4299: F, t1492: F, t15562: F, t1094: F, t17449: F, t3058: F, t487: F, t870: F, t442: F, t8287: F, t444: F, t5239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17504 = 0.58482233974552040708e0 * t1102 * t17502;
    let t17515 = t4300 * t5110;
    let t17516 = t4299 * t17515;
    let t17527 = 0.17544670192365612213e1 * t15562 * t1492;
    let t17529 = t3058 * t17449 * t1094;
    let t17531 = 0.35089340384731224426e1 * t1102 * t17529;
    let t17533 = t487 * t870;
    let t17534 = t442 * t442;
    let t17536 = t8287 * t17533 * t17534;
    let t17537 = t5239 * t444;
    (t17504, t17515, t17516, t17527, t17529, t17531, t17534, t17536, t17537)
}
