//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1291/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1291(t14547: f64, t20842: f64, t27363: f64, t51274: f64, t8906: f64, t14046: f64, t3172: f64, t14565: f64, t346: f64, t838: f64, t859: f64, t27823: f64, t3139: f64, t4028: f64) -> (f64, f64, f64, f64, f64) {
    let t54391 = t14547 * t20842 * t27363;
    let t54394 = t51274 * t8906;
    let t54397 = t14046 * t3172;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54404 = t4028 * t3139 * t27823;
    (t54391, t54394, t54397, t54401, t54404)
}
