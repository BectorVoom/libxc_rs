//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1197/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1197(t1679: f64, t619: f64, t615: f64, t77: f64, t2049: f64, t84: f64, t1985: f64, t578: f64, t1993: f64, t112: f64, t234: f64, t599: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18351 = t1679 * t619;
    let t18356 = t77 * t615 * t619;
    let t18360 = t77 * t84 * t2049;
    let t18363 = t578 * t1985;
    let t18366 = t578 * t1993;
    let t18392 = t234 * t112;
    let t18394 = t599 * t630;
    (t18351, t18356, t18360, t18363, t18366, t18392, t18394)
}
