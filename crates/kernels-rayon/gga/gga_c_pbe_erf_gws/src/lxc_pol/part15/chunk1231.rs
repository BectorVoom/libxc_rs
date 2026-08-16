//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1231/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1231(t14423: f64, t14682: f64, t3989: f64, t6360: f64, t14711: f64, t8801: f64, t13888: f64, t14651: f64, t14791: f64, t2408: f64, t29751: f64, t3066: f64, t3207: f64, t50944: f64, t52982: f64, t52986: f64, t52989: f64, t52992: f64, t52994: f64, t52997: f64, t53009: f64, t53012: f64, t53015: f64, t9213: f64, t9283: f64, t9321: f64) -> f64 {
    let t53019 = t3989 * t14682 * t14423 * t6360;
    let t53025 = 7.0_f64 / 24.0_f64 * t8801 * t14711;
    let t53026 = -t52982 / 192.0_f64 - t52986 / 192.0_f64 + 7.0_f64 / 144.0_f64 * t50944 + t52989 + t52992 - t52994 / 24.0_f64 - t52997 / 24.0_f64 + t3207 * t9283 * t13888 * t9213 / 8.0_f64 - t3066 * t9283 * t14791 * t9321 / 16.0_f64 - t53009 / 1536.0_f64 - t53012 + 35.0_f64 / 432.0_f64 * t53015 + t53019 / 1536.0_f64 - t2408 * t29751 * t14651 / 12.0_f64 + t53025;
    t53026
}
