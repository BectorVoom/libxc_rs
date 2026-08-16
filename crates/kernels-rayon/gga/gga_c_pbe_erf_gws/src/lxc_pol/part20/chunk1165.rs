//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1165/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1165(t14138: f64, t14733: f64, t1173: f64, t3202: f64, t13973: f64, t14706: f64, t14708: f64, t14711: f64, t14714: f64, t14716: f64, t14718: f64, t14722: f64, t14727: f64, t14729: f64, t14731: f64, t3207: f64) -> f64 {
    let t14734 = t14733 * t14138;
    let t14737 = t1173 * t3202;
    let t14739 = t14706 / 768.0_f64 - 7.0_f64 / 288.0_f64 * t14708 - t3207 * t14711 / 16.0_f64 - t14714 / 48.0_f64 - 7.0_f64 / 2304.0_f64 * t14716 + 7.0_f64 / 288.0_f64 * t14718 - t14722 / 3072.0_f64 + t14727 / 3072.0_f64 + t14729 / 48.0_f64 + t14731 / 16.0_f64 - t14734 / 96.0_f64 + 7.0_f64 / 4608.0_f64 * t13973 + t14737 / 96.0_f64;
    t14739
}
