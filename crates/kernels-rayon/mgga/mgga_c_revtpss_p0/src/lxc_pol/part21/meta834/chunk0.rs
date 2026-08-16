//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3122/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3122(t480: f64, t57465: f64, t1235: f64, t1789: f64, t2434: f64, t371: f64, t12987: f64, t1803: f64, t1261: f64, t16746: f64, t247: f64, t3634: f64) -> (f64, f64, f64, f64) {
    let t57466 = t57465 * t480;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57473 = t12987 * t1803;
    let t57478 = t1261 * t247 * t3634 * t16746;
    (t57466, t57471, t57473, t57478)
}
