//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1028/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1028<F: Float>(t20689: F, t6339: F, t19561: F, t274: F, t346: F, t6161: F, t814: F, t2121: F, t337: F, t5: F, t822: F, t6253: F, t6563: F, t2100: F, t816: F, t2074: F, t2157: F) -> (F, F, F, F, F, F, F) {
    let t20690 = t6339 * t20689;
    let t20691 = 7.0 / 24.0 * t20690;
    let t20692 = t19561 * t274;
    let t20693 = t20692 * t346;
    let t20695 = t6161 * t814;
    let t20698 = t2121 * t337 * t5 * t20695;
    let t20700 = t822 * t20693 * t20698 / 16.0;
    let t20702 = 3.0 / 8.0 * t6253 * t6563;
    let t20703 = t816 * t2100;
    let t20708 = t2157 * t2074;
    (t20691, t20692, t20695, t20700, t20702, t20703, t20708)
}
