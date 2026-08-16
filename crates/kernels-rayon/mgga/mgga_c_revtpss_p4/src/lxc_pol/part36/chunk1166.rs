//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1166/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1166(t5816: f64, t84: f64, t77: f64, t2034: f64, t22475: f64, t2014: f64, t7898: f64, t7901: f64, t4248: f64, t7742: f64, t28172: f64, t7900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29561 = t84 * t5816;
    let t29562 = t77 * t29561;
    let t29576 = t2034 * t22475;
    let t29578 = 2.0_f64 * t2014 * t29576;
    let t29580 = 6.0_f64 * t7898 * t7901;
    let t29582 = 4.0_f64 * t4248 * t7742;
    let t29583 = t28172 * t7900;
    (t29562, t29576, t29578, t29580, t29582, t29583)
}
