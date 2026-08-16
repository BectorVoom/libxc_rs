//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1056/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1056(t2277: f64, t2343: f64, t6545: f64, t902: f64, t9105: f64, t9110: f64, t9113: f64, t9114: f64, t9118: f64, t9121: f64, t9123: f64, t9570: f64, t9575: f64, t9579: f64, t9581: f64) -> f64 {
    let t9584 = t9105 - t9110 - t9113 - t2277 * t9570 / 256.0_f64 - t9114 + 7.0_f64 / 2304.0_f64 * t6545 - t9118 + t9121 + t9123 + t902 * t9575 / 1536.0_f64 + t9579 + t2343 * t9581 / 384.0_f64;
    t9584
}
