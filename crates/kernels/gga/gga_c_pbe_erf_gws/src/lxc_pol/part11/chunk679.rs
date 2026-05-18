//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 679/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk679<F: Float>(t155: F, t506: F, t133: F, t8199: F, t1368: F, t285: F, t991: F, t281: F, t3013: F, t545: F, t39: F, t159: F) -> (F, F, F, F, F, F, F) {
    let t8236 = t155 * t506;
    let t8252 = t133 * t8199;
    let t8269 = t991 * t1368 * t285;
    let t8270 = t281 * t8269;
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    (t8236, t8252, t8269, t8270, t8277, t8279, t8281)
}
