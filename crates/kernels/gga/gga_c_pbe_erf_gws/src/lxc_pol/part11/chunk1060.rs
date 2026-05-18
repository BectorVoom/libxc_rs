//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1060/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1060<F: Float>(t13379: F, t6627: F, t13433: F, t9630: F, t13599: F, t6501: F, t11592: F, t11868: F, t13496: F, t6484: F, t13371: F, t6542: F) -> (F, F, F, F, F, F) {
    let t46013 = t6627 * t13379;
    let t46023 = t9630 * t13433;
    let t46078 = t6501 * t13599;
    let t46098 = t11592 * t11868;
    let t46104 = t6484 * t13496;
    let t46115 = t6542 * t13371;
    (t46013, t46023, t46078, t46098, t46104, t46115)
}
