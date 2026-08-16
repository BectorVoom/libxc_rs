//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 763/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk763(t1764: f64, t187: f64, t22: f64, t1679: f64, t586: f64, t1878: f64, t1648: f64, t1652: f64, t1683: f64, t633: f64, t1725: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5292 = 1.0_f64 / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5304 = t1679 * t586;
    let t5312 = t1878 * t586;
    let t5315 = t1648 * t1652;
    let t5317 = t633 * t1683;
    let t5322 = t582 * t1725;
    (t5293, t5304, t5312, t5315, t5317, t5322)
}
