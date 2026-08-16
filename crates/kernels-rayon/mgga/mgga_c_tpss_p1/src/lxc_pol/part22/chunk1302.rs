//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1302/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1302(t1791: f64, t62020: f64, t18351: f64, t5790: f64, t18350: f64, t31464: f64, t5784: f64, t18669: f64, t7690: f64, t18347: f64, t61938: f64, t61942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t62339 = t1791 * t62020;
    let t62342 = t5790 * t18351;
    let t62343 = t18350 * t62342;
    let t62345 = t31464 * t5784;
    let t62348 = t7690 * t18669;
    let t62349 = t62348 * t18347;
    let t62351 = t1791 * t61938;
    let t62356 = t1791 * t61942;
    (t62339, t62342, t62343, t62345, t62348, t62349, t62351, t62356)
}
