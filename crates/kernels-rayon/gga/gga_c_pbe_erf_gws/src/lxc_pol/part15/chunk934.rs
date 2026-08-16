//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 934/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk934(t1563: f64, t9: f64, t1504: f64, t967: f64, t155: f64, t506: f64, t2911: f64, t2913: f64, t2873: f64, t481: f64, t1533: f64, t133: f64, t8146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8231 = t9 * t1563;
    let t8232 = t967 * t1504;
    let t8236 = t155 * t506;
    let t8238 = t2911 * t8236 * t2913;
    let t8240 = t2873 * t481;
    let t8244 = t967 * t1533;
    let t8249 = 0.11495033333333333333e1_f64 * t133 * t8146;
    (t8231, t8232, t8238, t8240, t8244, t8249)
}
