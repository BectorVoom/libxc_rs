//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1142/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1142(t1125: f64, t14024: f64, t3139: f64, t9026: f64, t4028: f64, t14007: f64, t3261: f64, t14029: f64, t14506: f64, t14508: f64, t14510: f64, t14512: f64, t14514: f64, t14516: f64, t14518: f64) -> (f64, f64, f64) {
    let t14520 = t1125 * t14024;
    let t14522 = t3139 * t9026;
    let t14523 = t4028 * t14522;
    let t14525 = t14007 * t3261;
    let t14527 = -7.0_f64 / 1152.0_f64 * t14029 + 7.0_f64 / 1152.0_f64 * t14506 - t14508 / 96.0_f64 + t14510 / 48.0_f64 + t14512 / 48.0_f64 + t14514 / 48.0_f64 + 5.0_f64 / 384.0_f64 * t14516 + t14518 / 192.0_f64 - 7.0_f64 / 288.0_f64 * t14520 - t14523 / 96.0_f64 + t14525 / 384.0_f64;
    (t14520, t14522, t14527)
}
