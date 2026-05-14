//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 739/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk739<F: Float>(t7373: F, t7380: F, t935: F, t313: F, t2661: F, t7371: F, t2672: F, t24: F, t2602: F, t862: F, t2263: F, t864: F, t6534: F, t322: F, t530: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7381 = t7373 * t7380;
    let t7382 = t7381 * t935;
    let t7383 = t313 * t7382;
    let t7386 = t2661 * t7371;
    let t7387 = t7373 * t2672;
    let t7388 = t7387 * t935;
    let t7389 = t313 * t7388;
    let t7394 = t24 * t2602;
    let t7395 = t862 * t7394;
    let t7397 = t864 * t2263;
    let t7398 = t7397 * t6534;
    let t7399 = t322 * t7398;
    let t7402 = t530 * t866;
    (t7382, t7383, t7386, t7388, t7389, t7394, t7395, t7397, t7398, t7399, t7402)
}
