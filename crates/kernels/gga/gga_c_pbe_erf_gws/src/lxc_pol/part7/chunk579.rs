//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 579/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk579<F: Float>(t2242: F, t894: F, t2367: F, t2379: F, t2352: F, t810: F, t2376: F, t2409: F, t2233: F, t2246: F, t1327: F, t409: F, t1285: F, t1291: F, t1293: F, t403: F) -> (F, F, F, F, F, F, F, F) {
    let t4487 = t2242 * t894;
    let t4489 = t2367 * t2379;
    let t4491 = t2352 * t810;
    let t4493 = t2409 * t2376 * t4491;
    let t4496 = t2246 * t2233;
    let t4498 = t409 * t1327;
    let t4499 = 12.0 * t4498;
    let t4502 = t1291 * t1285 * t1293 * t403;
    (t4487, t4489, t4491, t4493, t4496, t4498, t4499, t4502)
}
