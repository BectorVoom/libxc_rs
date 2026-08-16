//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1271/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1271(t4182: f64, t6781: f64, t829: f64, t830: f64, t13791: f64, t3039: f64, t13984: f64, t14657: f64, t51714: f64, t13793: f64, t51584: f64, t13939: f64, t3040: f64, t51063: f64, t51561: f64, t51564: f64, t53664: f64, t53666: f64, t53668: f64, t53671: f64, t53675: f64, t53677: f64, t827: f64, t8793: f64) -> f64 {
    let t53679 = t6781 * t4182;
    let t53681 = t829 * t830 * t53679;
    let t53688 = t3039 * t13791;
    let t53689 = t53688 * t13984;
    let t53691 = t14657 * t51714;
    let t53693 = t53688 * t13793;
    let t53695 = t14657 * t51584;
    let t53697 = -t53664 / 384.0_f64 - t53666 - t53668 / 768.0_f64 - t53671 / 1536.0_f64 - t3040 * t13939 / 48.0_f64 + t53675 / 8.0_f64 - t53677 / 48.0_f64 - t827 * t53681 / 48.0_f64 - 7.0_f64 / 288.0_f64 * t51561 + 7.0_f64 / 1152.0_f64 * t51564 + t8793 * t51063 / 48.0_f64 - t53689 / 48.0_f64 - t53691 / 96.0_f64 - t53693 / 24.0_f64 + t53695 / 48.0_f64;
    t53697
}
