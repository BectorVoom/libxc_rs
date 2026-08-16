//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1297/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1297(t234: f64, t630: f64, t640: f64, t2073: f64, t599: f64, t2074: f64, t18394: f64, t2100: f64, t68: f64, t7594: f64, t5506: f64, t619: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61870 = t234 * t630;
    let t61871 = t61870 * t640;
    let t61873 = t599 * t2073;
    let t61874 = t61873 * t2074;
    let t61876 = t18394 * t2100;
    let t61877 = t68 * t7594;
    let t61938 = t5506 * t619;
    (t61870, t61871, t61873, t61874, t61876, t61877, t61938)
}
