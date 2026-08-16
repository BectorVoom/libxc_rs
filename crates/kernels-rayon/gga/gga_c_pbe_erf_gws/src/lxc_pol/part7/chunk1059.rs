//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1059/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1059(t1570: f64, t510: f64, t5651: f64, t1503: f64, t1592: f64, t142: f64, t524: f64, t5878: f64, t5649: f64, t1504: f64, t5870: f64, t1354: f64, t1368: f64, t281: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19129 = t5651 * t510 * t1570;
    let t19132 = t1503 * t1592;
    let t19136 = t524 * t5878 * t142;
    let t19138 = t1503 * t5649;
    let t19140 = t5651 * t1504 * t510;
    let t19143 = t5651 * t5870;
    let t19148 = t281 * t1354 * t1368 * t285;
    (t19129, t19132, t19136, t19138, t19140, t19143, t19148)
}
