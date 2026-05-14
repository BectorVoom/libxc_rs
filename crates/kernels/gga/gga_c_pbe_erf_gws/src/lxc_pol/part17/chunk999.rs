//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 999/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk999<F: Float>(t2409: F, t6149: F, t14121: F, t274: F, t837: F, t850: F, t851: F, t833: F, t3955: F, t894: F, t3975: F, t9521: F, t3972: F, t2118: F, t332: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14122 = t2409 * t6149;
    let t14123 = t14121 * t14122;
    let t14125 = t274 * t837;
    let t14127 = t850 * t851 * t14125;
    let t14128 = t14127 * t833;
    let t14129 = 7.0 / 144.0 * t14128;
    let t14130 = t3955 * t894;
    let t14131 = 7.0 / 144.0 * t14130;
    let t14132 = t3975 * t9521;
    let t14133 = t3972 * t14132;
    let t14135 = t2118 * t332;
    let t14136 = t822 * t14135;
    (t14122, t14123, t14125, t14127, t14128, t14129, t14130, t14131, t14132, t14133, t14135, t14136)
}
