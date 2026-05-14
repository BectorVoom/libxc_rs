//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 943/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk943<F: Float>(t2121: F, t337: F, t46173: F, t11563: F, t3916: F, t13300: F, t6484: F, t13349: F, t6627: F, t13363: F, t6416: F, t13242: F, t3116: F, t6331: F, t3786: F, t3912: F, t6158: F) -> (F, F, F, F, F, F, F) {
    let t46175 = t2121 * t337 * t46173;
    let t46199 = t3916 * t11563;
    let t46251 = t6484 * t13300;
    let t46253 = t6627 * t13349;
    let t46280 = t6416 * t13363;
    let t46324 = t3116 * t6331 * t13242;
    let t46327 = t3912 * t6158 * t3786;
    (t46175, t46199, t46251, t46253, t46280, t46324, t46327)
}
