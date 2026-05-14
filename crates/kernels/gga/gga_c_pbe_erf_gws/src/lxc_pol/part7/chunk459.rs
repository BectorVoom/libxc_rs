//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 459/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk459<F: Float>(t339: F, t816: F, t2080: F, t2084: F, t860: F, t1220: F, t1278: F, t1288: F, t1296: F, t1328: F, t1330: F, t1335: F, t1338: F, t1427: F, t1429: F, t1440: F, t1450: F, t2050: F) -> (F, F, F, F) {
    let t2085 = t816 * t339;
    let t2087 = t2080 * t2084 * t2085;
    let t2089 = t2087 * t860 / 96.0;
    let t2090 = t1220 + t1328 + t1330 + t1335 + t1338 - t2050 + t1427 - t1429 + t1450 - t1278 + t1288 + t1296 - t1440;
    (t2085, t2087, t2089, t2090)
}
