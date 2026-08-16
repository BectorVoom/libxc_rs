//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1152/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1152(t20571: f64, t20576: f64, t20582: f64, t20584: f64, t20588: f64, t20593: f64, t20594: f64, t20601: f64, t20606: f64, t20607: f64, t20608: f64, t20615: f64, t2255: f64, t2277: f64, t6276: f64, t6665: f64) -> f64 {
    let t20616 = t2277 * t2255 * t20571 * t6665 / 256.0_f64 + 7.0_f64 / 24.0_f64 * t20576 - t20582 + t20584 - t20588 - t20593 - 7.0_f64 / 144.0_f64 * t20594 + t20601 - t20606 - 3.0_f64 / 16.0_f64 * t20607 * t6276 * t20608 + t20615;
    t20616
}
