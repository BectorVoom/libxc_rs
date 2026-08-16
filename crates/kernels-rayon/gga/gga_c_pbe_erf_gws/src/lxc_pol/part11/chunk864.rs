//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 864/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk864(t13408: f64, t3131: f64, t6523: f64, t2168: f64, t11977: f64, t13520: f64, t13522: f64, t13527: f64, t13529: f64, t13531: f64, t13538: f64, t13541: f64, t13545: f64, t13549: f64, t13553: f64, t13557: f64, t13561: f64, t2277: f64, t2343: f64, t6275: f64, t6637: f64, t914: f64, t929: f64) -> (f64, f64, f64) {
    let t13565 = t6523 * t3131 * t13408;
    let t13567 = 3.0_f64 / 16.0_f64 * t2168 * t13565;
    let t13568 = t13520 + 7.0_f64 / 384.0_f64 * t11977 - t13522 + t13527 + t13529 + t2343 * t13531 / 128.0_f64 + t13538 - t2277 * t13541 / 768.0_f64 + t6275 * t13545 / 32.0_f64 + t6637 * t13549 / 256.0_f64 + 5.0_f64 / 256.0_f64 * t929 * t13553 - t914 * t13557 / 1536.0_f64 + t2343 * t13561 / 128.0_f64 - t13567;
    (t13565, t13567, t13568)
}
