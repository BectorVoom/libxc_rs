//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 615/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk615<F: Float>(t296: F, t413: F, t6073: F, t816: F, t322: F, t897: F, t2209: F, t337: F, t2118: F, t2365: F, t274: F, t4394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0 * t6075;
    let t6094 = t816 * t816;
    let t6095 = 1.0 / t6094;
    let t6096 = t322 * t6095;
    let t6125 = t897 * t897;
    let t6126 = 1.0 / t6125;
    let t6148 = t2209 * t337;
    let t6154 = t2118 * t2365;
    let t6158 = t4394 * t274;
    (t6074, t6075, t6076, t6094, t6095, t6096, t6125, t6126, t6148, t6154, t6158)
}
