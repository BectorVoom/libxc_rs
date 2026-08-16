//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 694/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk694(t1592: f64, t475: f64, t1503: f64, t522: f64, t142: f64, t1504: f64, t525: f64, t1354: f64, t285: f64, t545: f64, t281: f64, t1368: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5598 = t475 * t1592;
    let t5601 = t1503 * t522;
    let t5602 = t142 * t1504;
    let t5603 = t525 * t5602;
    let t5607 = t1354 * t545 * t285;
    let t5608 = t281 * t5607;
    let t5611 = t535 * t1368 * t285;
    (t5598, t5601, t5602, t5603, t5607, t5608, t5611)
}
