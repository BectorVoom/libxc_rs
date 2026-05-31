//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1157/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1157<F: Float>(t6402: F, t6633: F, t6183: F, t6341: F, t6339: F, t19561: F, t274: F, t346: F, t6161: F, t814: F, t2121: F, t337: F, t5: F) -> (F, F, F, F, F, F) {
    let t20687 = t6402 * t6633;
    let t20689 = t6183 * t6341;
    let t20690 = t6339 * t20689;
    let t20691 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t20690;
    let t20692 = t19561 * t274;
    let t20693 = t20692 * t346;
    let t20695 = t6161 * t814;
    let t20698 = t2121 * t337 * t5 * t20695;
    (t20687, t20691, t20692, t20693, t20695, t20698)
}
