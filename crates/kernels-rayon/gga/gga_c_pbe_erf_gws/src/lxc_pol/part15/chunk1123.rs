//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1123/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1123(t14136: f64, t14138: f64, t1173: f64, t2222: f64, t13977: f64, t13981: f64, t13985: f64, t13989: f64, t13991: f64, t13996: f64, t13999: f64, t14003: f64, t14109: f64, t14115: f64, t14119: f64, t14123: f64, t14129: f64, t14131: f64, t14133: f64, t2408: f64, t3066: f64, t335: f64) -> f64 {
    let t14139 = t14136 * t14138;
    let t14141 = t1173 * t2222;
    let t14143 = -t13977 / 96.0_f64 + t2408 * t13981 / 48.0_f64 - t13985 / 48.0_f64 + t13989 + t3066 * t13991 / 24.0_f64 + t3066 * t13996 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t13999 + t14003 - t335 * t14109 / 96.0_f64 + t14115 + t14119 / 1536.0_f64 + t14123 / 16.0_f64 - t14129 - t14131 - t14133 / 1536.0_f64 - t14139 / 96.0_f64 + t14141 / 96.0_f64;
    t14143
}
