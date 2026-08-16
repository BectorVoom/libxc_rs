//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1336/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1336(t11689: f64, t14007: f64, t14535: f64, t3108: f64, t11953: f64, t14015: f64, t54237: f64, t54239: f64, t57060: f64, t57062: f64, t57064: f64, t57066: f64, t57068: f64, t57070: f64, t57073: f64) -> f64 {
    let t57075 = t14007 * t11689;
    let t57077 = t3108 * t14535;
    let t57079 = t14015 * t11953;
    let t57081 = -t57060 / 24.0_f64 - t57062 / 192.0_f64 - t57064 / 48.0_f64 + t57066 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t57068 + t57070 / 192.0_f64 + t54237 - t57073 / 96.0_f64 - t57075 / 192.0_f64 - t57077 / 24.0_f64 - t54239 - t57079 / 96.0_f64;
    t57081
}
