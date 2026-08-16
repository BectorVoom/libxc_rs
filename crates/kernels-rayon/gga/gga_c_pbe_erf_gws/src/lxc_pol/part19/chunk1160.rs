//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1160/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1160(t1205: f64, t2494: f64, t2376: f64, t2409: f64, t1144: f64, t338: f64, t4111: f64, t14611: f64, t1161: f64, t4110: f64, t3067: f64, t14283: f64, t14609: f64, t14615: f64, t14618: f64, t14902: f64, t14906: f64, t14911: f64, t14914: f64, t14918: f64, t2408: f64, t3066: f64, t335: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14922 = t1205 * t2494;
    let t14924 = t2409 * t2376 * t14922;
    let t14928 = t338 * t1144 * t4111;
    let t14931 = 7.0_f64 / 2304.0_f64 * t14611;
    let t14935 = t4110 * t1161;
    let t14937 = t2409 * t3067 * t14935;
    let t14940 = t3066 * t14902 / 48.0_f64 + t2408 * t14906 / 48.0_f64 - t827 * t14911 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t14914 - t827 * t14918 / 96.0_f64 - t14609 / 1536.0_f64 + t2408 * t14924 / 48.0_f64 - t335 * t14928 / 96.0_f64 + t14931 - t14615 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t14283 - t14618 / 48.0_f64 + t3066 * t14937 / 48.0_f64;
    (t14922, t14924, t14928, t14935, t14937, t14940)
}
