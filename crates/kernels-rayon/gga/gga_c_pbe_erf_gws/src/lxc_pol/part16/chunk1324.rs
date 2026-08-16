//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1324/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1324(t14911: f64, t2367: f64, t353: f64, t4228: f64, t4386: f64, t810: f64, t53625: f64, t1115: f64, t14311: f64, t14327: f64, t14888: f64, t14894: f64, t20113: f64, t22134: f64, t29751: f64, t3040: f64, t3207: f64, t4083: f64, t51526: f64, t52345: f64, t52480: f64, t53599: f64, t53601: f64, t53623: f64, t6793: f64, t8634: f64) -> f64 {
    let t55279 = 7.0_f64 / 144.0_f64 * t2367 * t14911;
    let t55284 = t4386 * t353 * t4228 * t810;
    let t55290 = 7.0_f64 / 576.0_f64 * t53625;
    let t55294 = -t3207 * t29751 * t14894 / 8.0_f64 + 7.0_f64 / 288.0_f64 * t52345 + t53599 / 12.0_f64 + t53601 / 24.0_f64 - t8634 * t4083 / 48.0_f64 - t3040 * t14311 / 48.0_f64 - t3040 * t14327 / 48.0_f64 + t55279 - t1115 * t52480 / 96.0_f64 + t6793 * t55284 / 24.0_f64 + t20113 * t14888 / 48.0_f64 - t53623 / 768.0_f64 + t55290 - t22134 * t4083 / 96.0_f64 + 7.0_f64 / 1152.0_f64 * t51526;
    t55294
}
