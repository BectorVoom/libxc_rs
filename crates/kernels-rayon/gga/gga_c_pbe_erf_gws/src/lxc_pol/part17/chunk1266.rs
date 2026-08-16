//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1266/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1266(t3965: f64, t9323: f64, t14460: f64, t4414: f64, t1192: f64, t13911: f64, t19631: f64, t22379: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t4155: f64, t51524: f64, t53572: f64, t53578: f64, t53579: f64, t53581: f64, t53584: f64, t53585: f64, t53595: f64, t53598: f64, t53599: f64, t9688: f64) -> f64 {
    let t53601 = t3965 * t9323;
    let t53610 = 7.0_f64 / 72.0_f64 * t4414 * t14460;
    let t53613 = -t53572 / 24.0_f64 - t53578 - t53579 / 48.0_f64 - t53581 / 48.0_f64 - t53584 + 35.0_f64 / 216.0_f64 * t53585 + t2408 * t2409 * t19631 * t4155 / 48.0_f64 - 5.0_f64 / 128.0_f64 * t53595 - t53598 + t53599 / 24.0_f64 + t53601 / 48.0_f64 + t3066 * t2409 * t3067 * t1192 * t9688 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t51524 - t53610 + t22379 * t13911 / 24.0_f64;
    t53613
}
