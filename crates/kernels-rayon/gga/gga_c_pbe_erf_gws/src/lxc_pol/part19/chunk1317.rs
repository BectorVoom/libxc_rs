//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1317/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1317(t11689: f64, t14007: f64, t14535: f64, t3108: f64, t11953: f64, t14015: f64, t11803: f64, t3065: f64, t36897: f64, t858: f64, t9119: f64, t11648: f64, t14101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57075 = t14007 * t11689;
    let t57077 = t3108 * t14535;
    let t57079 = t14015 * t11953;
    let t57082 = t14007 * t11803;
    let t57086 = t9119 * t3065 * t858 * t36897;
    let t57088 = t14101 * t11648;
    (t57075, t57077, t57079, t57082, t57086, t57088)
}
