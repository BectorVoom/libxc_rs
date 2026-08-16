//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1340/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1340(t54267: f64, t54271: f64, t54283: f64, t54285: f64, t54289: f64, t51372: f64, t54265: f64, t54269: f64, t54273: f64, t54276: f64, t54280: f64, t54287: f64) -> f64 {
    let t55562 = 7.0_f64 / 36.0_f64 * t54267;
    let t55564 = 7.0_f64 / 72.0_f64 * t54271;
    let t55569 = 7.0_f64 / 288.0_f64 * t54283;
    let t55570 = 7.0_f64 / 72.0_f64 * t54285;
    let t55572 = 7.0_f64 / 72.0_f64 * t54289;
    let t55573 = -t54265 / 48.0_f64 + t55562 + t54269 / 24.0_f64 - t55564 + t54273 / 96.0_f64 + t54276 / 4.0_f64 - t54280 / 32.0_f64 - 7.0_f64 / 72.0_f64 * t51372 + t55569 - t55570 - t54287 / 384.0_f64 - t55572;
    t55573
}
