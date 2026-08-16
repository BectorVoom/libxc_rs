//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1357/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1357(t15492: f64, t8801: f64, t11407: f64, t12220: f64, t14327: f64, t14881: f64, t15482: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t3921: f64, t52159: f64, t53015: f64, t54937: f64, t54942: f64, t54946: f64, t55729: f64, t56197: f64, t56206: f64, t56209: f64, t56236: f64, t56240: f64, t56242: f64, t810: f64, t9283: f64) -> f64 {
    let t58028 = t8801 * t15492;
    let t58035 = 35.0_f64 / 108.0_f64 * t53015 - t56197 / 96.0_f64 - t3066 * t9283 * t14881 * t11407 / 8.0_f64 - t54937 - t54942 - t56206 / 192.0_f64 + t56209 / 384.0_f64 - t3921 * t14327 / 96.0_f64 - t54946 + t2408 * t2409 * t2376 * t15482 * t810 / 48.0_f64 - t56236 / 6.0_f64 + 7.0_f64 / 48.0_f64 * t58028 + t56240 / 768.0_f64 + 35.0_f64 / 216.0_f64 * t52159 - t12220 * t55729 / 96.0_f64 + 7.0_f64 / 576.0_f64 * t56242;
    t58035
}
