//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 848/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk848<F: Float>(t6587: F, t899: F, t912: F, t6401: F, t6684: F, t19561: F, t274: F, t346: F, t2251: F, t2300: F, t2250: F, t2170: F, t332: F, t2332: F, t336: F, t9239: F) -> (F, F, F, F, F, F, F, F) {
    let t20646 = t899 * t912 * t6587;
    let t20675 = t6684 * t6401;
    let t20692 = t19561 * t274;
    let t20693 = t20692 * t346;
    let t20732 = t2251 * t2300;
    let t20733 = t2250 * t20732;
    let t20833 = t332 * t2170;
    let t20839 = t899 * t912 * t2332;
    let t20842 = t9239 * t336;
    (t20646, t20675, t20692, t20693, t20733, t20833, t20839, t20842)
}
