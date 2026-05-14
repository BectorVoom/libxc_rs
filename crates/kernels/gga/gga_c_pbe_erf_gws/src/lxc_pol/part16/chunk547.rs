//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 547/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk547<F: Float>(t329: F, t332: F, t369: F, t2182: F, t376: F, t353: F, t338: F, t2169: F) -> (F, F, F, F, F) {
    let t2401 = t329 * t332 * t369;
    let t2402 = t376 * t2182;
    let t2403 = t353 * t2402;
    let t2404 = t338 * t2403;
    let t2407 = t332 * t2169;
    (t2401, t2402, t2403, t2404, t2407)
}
