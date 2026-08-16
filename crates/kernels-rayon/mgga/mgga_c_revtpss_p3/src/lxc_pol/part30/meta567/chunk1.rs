//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2015/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2015(t68: f64, t785: f64, t251: f64, t281: f64, t25410: f64, t7078: f64, t2453: f64, t2458: f64, t7049: f64, t1950: f64, t2769: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t93238 = t68 * t785;
    let t93240 = t281 * t93238 * t251;
    let t93242 = t93240 * t25410 * t7078;
    let t93252 = t2453 * t7049 * t2458;
    let t93261 = t786 * t1950 * t2769;
    (t93238, t93240, t93242, t93252, t93261)
}
