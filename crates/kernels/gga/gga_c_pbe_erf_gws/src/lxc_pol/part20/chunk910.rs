//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 910/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk910<F: Float>(t11196: F, t11197: F, t11200: F, t11202: F, t11205: F, t11206: F, t11211: F, t11212: F, t11216: F, t11219: F, t11221: F, t11222: F, t11225: F, t11233: F, t11235: F, t11236: F) -> (F,) {
    let t11240 = t11196 + t11197 + t11200 + t11202 + t11205 + t11206 + t11211 + t11212 + t11216 + t11219 + t11221 + t11222 + t11225 + t11233 + t11235 + t11236;
    (t11240,)
}
