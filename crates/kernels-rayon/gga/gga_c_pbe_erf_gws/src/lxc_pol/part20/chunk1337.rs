//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1337/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1337(t11803: f64, t14007: f64, t3065: f64, t36897: f64, t858: f64, t9119: f64, t11648: f64, t14101: f64, t11470: f64, t4028: f64, t11970: f64, t14011: f64) -> (f64, f64, f64, f64, f64) {
    let t57082 = t14007 * t11803;
    let t57086 = t9119 * t3065 * t858 * t36897;
    let t57088 = t14101 * t11648;
    let t57090 = t4028 * t11470;
    let t57092 = t14011 * t11970;
    (t57082, t57086, t57088, t57090, t57092)
}
