//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 200/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk200<F: Float>(t1: F, t231: F, t369: F, t46: F, t382: F, t132: F, t283: F) -> (F, F, F, F, F) {
    let t725 = t231 * t1;
    let t727 = 0.18311555036753159941e-3 * t725 * t369;
    let t728 = t231 * t46;
    let t730 = 0.58482233974552040708e0 * t728 * t382;
    let t731 = t132 * t283;
    (t725, t727, t728, t730, t731)
}
