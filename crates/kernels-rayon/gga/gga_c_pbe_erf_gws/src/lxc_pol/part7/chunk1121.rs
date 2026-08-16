//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1121/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1121(t353: f64, t745: f64, t859: f64, t939: f64, t19592: f64, t20081: f64, t20086: f64, t20092: f64, t20106: f64, t20108: f64, t20110: f64, t20113: f64, t20117: f64, t20121: f64, t20124: f64, t2074: f64, t2373: f64, t2382: f64, t2408: f64, t2409: f64, t2417: f64, t3067: f64, t335: f64, t338: f64, t4390: f64, t6724: f64, t6797: f64, t6816: f64, t6817: f64, t833: f64, t892: f64) -> f64 {
    let t20127 = t859 * t353 * t939 * t745;
    let t20130 = -455.0_f64 / 324.0_f64 * t20081 + t19592 * t4390 / 6.0_f64 + t2382 * t20086 * t833 / 32.0_f64 + 35.0_f64 / 18.0_f64 * t20092 - t2408 * t2409 * t3067 * t2074 * t2417 / 4.0_f64 - t335 * t338 * t892 * t6724 / 24.0_f64 - t6816 * t338 * t892 * t6817 - 35.0_f64 / 18.0_f64 * t20106 - 7.0_f64 / 6.0_f64 * t20108 - 7.0_f64 / 12.0_f64 * t20110 + t20113 * t6797 / 4.0_f64 - t20117 * t2373 / 4.0_f64 - t20121 * t2373 / 4.0_f64 - t20124 * t20127 / 8.0_f64;
    t20130
}
