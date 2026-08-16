//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1361/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1361(t1115: f64, t11342: f64, t1205: f64, t12213: f64, t14311: f64, t14911: f64, t15025: f64, t2376: f64, t2408: f64, t2409: f64, t3040: f64, t3066: f64, t3067: f64, t3306: f64, t3913: f64, t4083: f64, t4227: f64, t52251: f64, t55114: f64, t55117: f64, t55145: f64, t55764: f64, t56362: f64, t56366: f64, t56374: f64, t56404: f64, t56431: f64, t9807: f64) -> f64 {
    let t58140 = -t1115 * t55764 / 48.0_f64 - t3040 * t14911 / 48.0_f64 - t3913 * t14311 / 96.0_f64 + t56362 / 24.0_f64 + t55114 + t55117 + t56366 / 384.0_f64 + t3066 * t2409 * t3067 * t4227 * t3306 / 24.0_f64 + t2408 * t2409 * t2376 * t1205 * t9807 / 48.0_f64 - 5.0_f64 / 384.0_f64 * t56374 + 35.0_f64 / 216.0_f64 * t52251 + t3066 * t2409 * t12213 * t15025 / 24.0_f64 - 35.0_f64 / 216.0_f64 * t55145 - t11342 * t4083 / 96.0_f64 - 5.0_f64 / 384.0_f64 * t56404 - t56431 / 768.0_f64;
    t58140
}
