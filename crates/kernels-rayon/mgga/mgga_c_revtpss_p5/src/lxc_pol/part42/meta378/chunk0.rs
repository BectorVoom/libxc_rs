//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1244/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1244(t6132: f64, t698: f64, t6135: f64, t18946: f64, t930: f64, t141: f64, t6138: f64, t18942: f64, t18937: f64, t2908: f64, t11134: f64, t11366: f64, t11479: f64, t11480: f64, t18948: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19006 = t930 * t18946;
    let t19007 = t141 * t19006;
    let t19009 = t698 * t6138;
    let t19013 = t930 * t18942;
    let t19014 = t141 * t19013;
    let t19016 = t2908 * t18937;
    let t19017 = t141 * t19016;
    let t19019 = -0.301925e0_f64 * t18948 - t11479 - t11480 + 0.18396666666666666667e-1_f64 * t19002 - 0.11038e0_f64 * t19004 - 0.82785e-1_f64 * t19007 + 0.5519e-1_f64 * t19009 - 0.13418888888888888889e0_f64 * t11134 - 0.91983333333333333333e-1_f64 * t11366 + 0.16557e0_f64 * t19014 - 0.27595e-1_f64 * t19017;
    (t19002, t19004, t19007, t19009, t19014, t19017, t19019)
}
