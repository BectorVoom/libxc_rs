//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 477/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk477<F: Float>(t2087: F, t860: F, t1220: F, t1278: F, t1288: F, t1296: F, t1328: F, t1330: F, t1335: F, t1338: F, t1427: F, t1429: F, t1440: F, t1450: F, t2050: F, t19: F, t793: F, t796: F) -> (F, F, F) {
    let t2089 = t2087 * t860 / 96.0;
    let t2090 = t1220 + t1328 + t1330 + t1335 + t1338 - t2050 + t1427 - t1429 + t1450 - t1278 + t1288 + t1296 - t1440;
    let t2092 = t793 * t796 * t19;
    (t2089, t2090, t2092)
}
