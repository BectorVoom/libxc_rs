//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1325/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1325(t1205: f64, t20173: f64, t53645: f64, t14918: f64, t2367: f64, t1115: f64, t14185: f64, t14321: f64, t2408: f64, t3066: f64, t35566: f64, t51530: f64, t52350: f64, t52353: f64, t53631: f64, t53639: f64, t53643: f64, t53664: f64, t53668: f64, t53671: f64, t53675: f64, t9283: f64, t9297: f64, t9702: f64) -> f64 {
    let t55297 = t20173 * t1205;
    let t55311 = 7.0_f64 / 72.0_f64 * t53645;
    let t55315 = 7.0_f64 / 144.0_f64 * t2367 * t14918;
    let t55321 = -35.0_f64 / 216.0_f64 * t52353 - t53631 / 192.0_f64 + t3066 * t9283 * t55297 * t9297 / 4.0_f64 + t53639 / 1536.0_f64 - t2408 * t35566 * t14321 / 12.0_f64 - t2408 * t9283 * t14185 * t9702 / 12.0_f64 + t53643 / 768.0_f64 - t55311 - t1115 * t52350 / 96.0_f64 + t55315 - t53664 / 192.0_f64 - 119.0_f64 / 864.0_f64 * t51530 - t53668 / 384.0_f64 - t53671 / 768.0_f64 + t53675 / 4.0_f64;
    t55321
}
