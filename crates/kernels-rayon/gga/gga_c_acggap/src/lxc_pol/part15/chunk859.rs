//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 859/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk859(t1941: f64, t467: f64, t301: f64, t1713: f64, t157: f64, t1772: f64, t406: f64, t1524: f64, t524: f64, t1410: f64, t1753: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24794 = t1941 * t467;
    let t24811 = t1941 * t301;
    let t24893 = t1713 * t467;
    let t25706 = t1772 * t406 * t157;
    let t25727 = t1524 * t524 * t157;
    let t25732 = t1753 * t1410;
    let t25742 = t513 * t1410 * t157;
    (t24794, t24811, t24893, t25706, t25727, t25732, t25742)
}
