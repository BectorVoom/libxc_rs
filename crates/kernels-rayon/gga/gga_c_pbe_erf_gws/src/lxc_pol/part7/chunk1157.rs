//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1157/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1157(t6402: f64, t6633: f64, t6183: f64, t6341: f64, t6339: f64, t19561: f64, t274: f64, t346: f64, t6161: f64, t814: f64, t2121: f64, t337: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20687 = t6402 * t6633;
    let t20689 = t6183 * t6341;
    let t20690 = t6339 * t20689;
    let t20691 = 7.0_f64 / 24.0_f64 * t20690;
    let t20692 = t19561 * t274;
    let t20693 = t20692 * t346;
    let t20695 = t6161 * t814;
    let t20698 = t2121 * t337 * t5 * t20695;
    (t20687, t20691, t20692, t20693, t20695, t20698)
}
