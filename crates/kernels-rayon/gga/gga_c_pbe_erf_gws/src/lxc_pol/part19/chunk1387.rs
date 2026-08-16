//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1387/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1387(t55562: f64, t55564: f64, t55569: f64, t55570: f64, t55572: f64, t57108: f64, t57110: f64, t57112: f64, t57114: f64, t57117: f64, t57119: f64, t57121: f64, t57123: f64) -> f64 {
    let t58719 = -t57108 / 192.0_f64 - t57110 / 32.0_f64 - 7.0_f64 / 144.0_f64 * t57112 + 3.0_f64 / 128.0_f64 * t57114 - t57117 / 4.0_f64 + t55562 + t57119 / 384.0_f64 - 7.0_f64 / 288.0_f64 * t57121 + t57123 / 24.0_f64 - t55564 + t55569 - t55570 - t55572;
    t58719
}
