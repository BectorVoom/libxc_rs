//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1055/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1055(t3232: f64, t6627: f64, t6502: f64, t6506: f64, t6508: f64, t6517: f64, t9084: f64, t9086: f64, t9090: f64, t9094: f64, t9096: f64, t9100: f64, t9101: f64) -> f64 {
    let t9565 = 7.0_f64 / 288.0_f64 * t6627 * t3232;
    let t9567 = -7.0_f64 / 768.0_f64 * t6502 - 119.0_f64 / 1728.0_f64 * t6506 + 7.0_f64 / 1152.0_f64 * t6508 - t9084 + t9086 + t9090 + t9094 - t9096 - t9565 + t9100 - t9101 + 7.0_f64 / 2304.0_f64 * t6517;
    t9567
}
