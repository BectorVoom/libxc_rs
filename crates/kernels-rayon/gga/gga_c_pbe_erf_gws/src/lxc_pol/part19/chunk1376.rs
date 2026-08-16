//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1376/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1376(t1206: f64, t353: f64, t3717: f64, t4386: f64, t14888: f64, t26958: f64, t15036: f64, t14185: f64, t3886: f64, t859: f64, t11375: f64, t14911: f64, t22379: f64, t2376: f64, t27047: f64, t335: f64, t338: f64, t3907: f64, t4111: f64, t55739: f64, t55741: f64, t55745: f64, t57386: f64, t57390: f64, t57393: f64, t57395: f64, t58050: f64, t6793: f64, t814: f64, t8654: f64) -> f64 {
    let t58553 = t4386 * t353 * t1206 * t3717;
    let t58556 = t26958 * t14888;
    let t58558 = t26958 * t15036;
    let t58562 = t859 * t353 * t14185 * t3886;
    let t58580 = t22379 * t14888 / 24.0_f64 + t6793 * t58553 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t58556 - 7.0_f64 / 72.0_f64 * t58558 + t6793 * t58562 / 48.0_f64 - t8654 * t14911 / 48.0_f64 + t57386 / 96.0_f64 - t11375 * t27047 * t2376 * t58050 * t814 / 48.0_f64 - t55739 + t55741 - t57390 / 8.0_f64 - t335 * t338 * t3907 * t4111 / 96.0_f64 + t57393 / 12.0_f64 + t55745 + t57395 / 24.0_f64;
    t58580
}
