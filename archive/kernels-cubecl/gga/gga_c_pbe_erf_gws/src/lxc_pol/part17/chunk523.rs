//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 523/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk523<F: Float>(t2220: F, t376: F, t338: F, t840: F, t894: F, t892: F, t939: F, t2074: F, t353: F, t941: F, t845: F, t2201: F, t329: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2221 = t2220 * t376;
    let t2222 = t338 * t2221;
    let t2225 = t840 * t894;
    let t2227 = t892 * t939;
    let t2228 = t338 * t2227;
    let t2231 = t376 * t2074;
    let t2232 = t353 * t2231;
    let t2233 = t338 * t2232;
    let t2236 = t840 * t941;
    let t2238 = t892 * t845;
    let t2239 = t338 * t2238;
    let t2242 = t329 * t2201;
    (t2222, t2225, t2227, t2228, t2231, t2232, t2233, t2236, t2238, t2239, t2242)
}
