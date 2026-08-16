//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3165/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165(t17769: f64, t3647: f64, t1235: f64, t371: f64, t5318: f64, t676: f64, t225: f64, t56331: f64, t1789: f64, t2434: f64, t1261: f64, t16746: f64, t247: f64, t3634: f64) -> (f64, f64, f64, f64, f64) {
    let t57451 = t3647 * t17769;
    let t57463 = t1235 * t371 * t676 * t5318;
    let t57465 = t56331 * t225;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57478 = t1261 * t247 * t3634 * t16746;
    (t57451, t57463, t57465, t57471, t57478)
}
