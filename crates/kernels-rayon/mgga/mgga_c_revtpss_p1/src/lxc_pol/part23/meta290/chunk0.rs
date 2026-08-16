//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1522/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1522(t1046: f64, t11262: f64, t1041: f64, t3140: f64, t989: f64, t3149: f64, t3160: f64, t2923: f64, t910: f64, t287: f64, t2922: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11273 = t989 * t3140;
    let t11274 = t11273 * t3149;
    let t11277 = t11273 * t3160;
    let t11294 = t910 * t2923;
    let t11298 = 1.0_f64 / t2922 / t287;
    let t11299 = t275 * t11298;
    (t11263, t11264, t11274, t11277, t11294, t11298, t11299)
}
