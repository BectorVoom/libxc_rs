//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1101/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1101(t5676: f64, t570: f64, t6171: f64, t1750: f64, t31824: f64, t1988: f64, t9573: f64, t1089: f64, t13067: f64, t598: f64, t9552: f64, t30412: f64, t30416: f64, t30422: f64, t30429: f64, t30444: f64, t30452: f64, t30457: f64, t30463: f64, t34383: f64, t34391: f64, t37066: f64, t37067: f64) -> f64 {
    let t39254 = t570 * t5676;
    let t39256 = t570 * t6171;
    let t39262 = t31824 * t1750;
    let t39264 = t1988 * t9573;
    let t39268 = t598 * t1089 * t13067 * t9552;
    let t39270 = 0.31448092289604152067e-2_f64 * t30412 - 0.12579236915841660827e-2_f64 * t30416 + t30422 - t39254 / 96.0_f64 - t39256 / 48.0_f64 + t30429 - 0.7862023072401038017e-3_f64 * t30444 + 0.31448092289604152068e-3_f64 * t30452 - 0.45017719023973223821e-2_f64 * t30457 - 0.47172138434406228102e-3_f64 * t30463 + 0.34299214494455789578e-2_f64 * t39262 + t34383 - t34391 + t37066 - t37067 + 0.64311027177104605458e-3_f64 * t39264 + 0.64311027177104605458e-3_f64 * t39268;
    t39270
}
