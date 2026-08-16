//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2833/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2833(t23167: f64, t243: f64, t10726: f64, t2661: f64, t2723: f64, t14586: f64, t18408: f64, t23334: f64, t61625: f64, t10850: f64, t221: f64, t23172: f64, t2485: f64) -> (f64, f64, f64, f64, f64) {
    let t76569 = t243 * t23167;
    let t76572 = t2661 * t10726 * t76569 * t2723;
    let t76583 = t2661 * t10726 * t18408 * t14586;
    let t76587 = t2661 * t10726 * t61625 * t23334;
    let t76591 = t10850 * t2485 * t221 * t23172;
    (t76569, t76572, t76583, t76587, t76591)
}
