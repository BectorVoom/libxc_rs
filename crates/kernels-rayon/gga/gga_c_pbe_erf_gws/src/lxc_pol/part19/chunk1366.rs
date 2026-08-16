//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1366/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1366(t1206: f64, t353: f64, t3703: f64, t8599: f64, t11354: f64, t14881: f64, t15537: f64, t22343: f64, t3066: f64, t55248: f64, t55251: f64, t55258: f64, t56593: f64, t56596: f64, t56599: f64, t56604: f64, t56613: f64, t56618: f64, t56626: f64, t56638: f64, t56642: f64, t6793: f64, t9283: f64) -> f64 {
    let t58264 = t8599 * t353 * t1206 * t3703;
    let t58280 = t22343 * t15537 / 96.0_f64 - t6793 * t58264 / 16.0_f64 + t56593 / 24.0_f64 - t55248 + t56596 / 768.0_f64 - t55251 - t55258 + t56599 / 48.0_f64 + t56604 / 192.0_f64 - t3066 * t9283 * t14881 * t11354 / 16.0_f64 - t56613 / 768.0_f64 + t56618 / 384.0_f64 - t56626 / 48.0_f64 - t56638 / 384.0_f64 - t56642 / 768.0_f64;
    t58280
}
