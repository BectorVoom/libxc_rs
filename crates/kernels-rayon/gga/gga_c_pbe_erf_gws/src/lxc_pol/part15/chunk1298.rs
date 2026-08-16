//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1298/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1298(t338: f64, t54055: f64, t8891: f64, t14011: f64, t9358: f64, t9406: f64, t14007: f64, t9443: f64, t14015: f64, t9470: f64, t9366: f64, t14093: f64, t8848: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54057 = t54055 * t338 * t8891;
    let t54059 = t14011 * t9358;
    let t54061 = t14011 * t9406;
    let t54063 = t14007 * t9443;
    let t54065 = t14015 * t9470;
    let t54067 = t14007 * t9366;
    let t54069 = t8848 * t14093;
    (t54057, t54059, t54061, t54063, t54065, t54067, t54069)
}
