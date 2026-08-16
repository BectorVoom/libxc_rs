//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1317/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1317(t20154: f64, t2376: f64, t4207: f64, t814: f64, t14327: f64, t3083: f64, t53353: f64, t1185: f64, t14181: f64, t14187: f64, t14192: f64, t27105: f64, t4385: f64, t51096: f64, t52249: f64, t52251: f64, t52299: f64, t53346: f64, t53351: f64, t53355: f64, t53357: f64, t53362: f64, t8629: f64, t8654: f64, t8776: f64) -> f64 {
    let t55110 = t20154 * t2376 * t4207 * t814;
    let t55114 = 7.0_f64 / 144.0_f64 * t3083 * t14327;
    let t55117 = 7.0_f64 / 144.0_f64 * t53353;
    let t55124 = -t8629 * t52299 / 24.0_f64 + t8654 * t1185 * t14187 / 24.0_f64 + t8654 * t27105 * t14181 / 24.0_f64 + t8776 * t1185 * t14192 / 32.0_f64 - t4385 * t55110 / 48.0_f64 + t55114 - t53346 / 768.0_f64 - t53351 / 768.0_f64 + t55117 + t53355 / 12.0_f64 + t53357 / 48.0_f64 + t53362 / 384.0_f64 - 7.0_f64 / 1152.0_f64 * t51096 - 7.0_f64 / 144.0_f64 * t52249 + 35.0_f64 / 108.0_f64 * t52251;
    t55124
}
