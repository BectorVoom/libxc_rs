//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1171/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1171(t1981: f64, t582: f64, t1679: f64, t619: f64, t112: f64, t234: f64, t599: f64, t630: f64, t640: f64, t2073: f64, t68: f64, t1695: f64, t17942: f64, t510: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18350 = t1981 * t582;
    let t18351 = t1679 * t619;
    let t18392 = t234 * t112;
    let t18394 = t599 * t630;
    let t18395 = t18394 * t640;
    let t18396 = 2.0_f64 / 3.0_f64 * t18395;
    let t18397 = t68 * t2073;
    let t18434 = t17942 * t510 * t1695;
    (t18350, t18351, t18392, t18394, t18396, t18397, t18434)
}
