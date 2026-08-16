//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1289/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1289(t12213: f64, t13622: f64, t21807: f64, t2409: f64, t3066: f64, t338: f64, t353: f64, t36244: f64, t36246: f64, t376: f64, t3917: f64, t44104: f64, t46703: f64, t46710: f64, t46712: f64, t46714: f64, t46717: f64, t46723: f64, t46731: f64, t50002: f64, t8793: f64, t9890: f64) -> f64 {
    let t50642 = 7.0_f64 / 36.0_f64 * t46703 - t3917 * t9890 / 8.0_f64 + 7.0_f64 / 24.0_f64 * t46710 + 7.0_f64 / 24.0_f64 * t46712 - 7.0_f64 / 24.0_f64 * t46714 - 7.0_f64 / 4.0_f64 * t46717 + 7.0_f64 / 36.0_f64 * t46723 - 7.0_f64 / 12.0_f64 * t46731 + t3066 * t2409 * t12213 * t13622 / 4.0_f64 + 5.0_f64 / 4.0_f64 * t21807 * t338 * t353 * t376 * t50002 + 35.0_f64 / 36.0_f64 * t36244 - 35.0_f64 / 18.0_f64 * t36246 + t8793 * t44104 / 2.0_f64;
    t50642
}
