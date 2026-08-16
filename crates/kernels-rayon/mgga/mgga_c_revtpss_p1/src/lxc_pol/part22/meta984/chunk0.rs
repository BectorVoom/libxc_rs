//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3334/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3334(t18936: f64, t2251: f64, t141: f64, t930: f64, t18969: f64, t698: f64, t18972: f64, t2258: f64, t6092: f64, t13312: f64, t4578: f64, t18281: f64, t2857: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63236 = t18936 * t2251;
    let t63238 = t141 * t930 * t63236;
    let t63240 = t698 * t18969;
    let t63242 = t698 * t18972;
    let t63244 = t6092 * t2258;
    let t63246 = t141 * t930 * t63244;
    let t63248 = t4578 * t13312;
    let t63250 = t141 * t930 * t63248;
    let t63253 = t2857 * t18281 * t606;
    (t63236, t63238, t63240, t63242, t63244, t63246, t63248, t63250, t63253)
}
