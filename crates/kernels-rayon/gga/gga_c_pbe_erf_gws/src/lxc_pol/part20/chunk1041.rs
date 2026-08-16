//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1041/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1041(t346: f64, t9847: f64, t1114: f64, t2124: f64, t274: f64, t3028: f64, t2255: f64, t3258: f64, t11640: f64, t11646: f64, t11650: f64, t11652: f64, t11656: f64, t11665: f64, t2277: f64, t3247: f64, t8927: f64, t9447: f64, t9457: f64, t9464: f64, t9474: f64) -> (f64, f64, f64) {
    let t11667 = t9847 * t346;
    let t11668 = t1114 * t11667;
    let t11670 = t11668 * t2124 / 96.0_f64;
    let t11671 = t274 * t3028;
    let t11673 = t2255 * t3258 * t11671;
    let t11676 = 3.0_f64 / 512.0_f64 * t3247 * t11640 - t8927 + t11646 - t11650 - t3247 * t11652 / 64.0_f64 + t2277 * t11656 / 384.0_f64 + t11665 + t9447 - 119.0_f64 / 3456.0_f64 * t9457 - t11670 - t9464 + t9474 - t2277 * t11673 / 1536.0_f64;
    (t11670, t11673, t11676)
}
