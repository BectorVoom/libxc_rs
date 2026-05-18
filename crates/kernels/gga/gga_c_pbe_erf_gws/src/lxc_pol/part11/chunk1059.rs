//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1059/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1059<F: Float>(t13586: F, t2319: F, t13394: F, t13437: F, t21245: F, t13259: F, t6203: F, t13156: F, t369: F, t13400: F, t20675: F, t13410: F, t6627: F) -> (F, F, F, F, F, F, F) {
    let t45852 = t2319 * t13586;
    let t45863 = t2319 * t13394;
    let t45882 = t21245 * t13437;
    let t45887 = t6203 * t13259;
    let t45901 = t13156 * t369;
    let t45974 = t20675 * t13400;
    let t45990 = t6627 * t13410;
    (t45852, t45863, t45882, t45887, t45901, t45974, t45990)
}
