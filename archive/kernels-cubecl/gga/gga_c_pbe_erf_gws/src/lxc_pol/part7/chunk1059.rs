//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1059/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1059<F: Float>(t1570: F, t510: F, t5651: F, t1503: F, t1592: F, t142: F, t524: F, t5878: F, t5649: F, t1504: F, t5870: F, t1354: F, t1368: F, t281: F, t285: F) -> (F, F, F, F, F, F, F) {
    let t19129 = t5651 * t510 * t1570;
    let t19132 = t1503 * t1592;
    let t19136 = t524 * t5878 * t142;
    let t19138 = t1503 * t5649;
    let t19140 = t5651 * t1504 * t510;
    let t19143 = t5651 * t5870;
    let t19148 = t281 * t1354 * t1368 * t285;
    (t19129, t19132, t19136, t19138, t19140, t19143, t19148)
}
