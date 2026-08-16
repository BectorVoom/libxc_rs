//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2715/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2715(t17361: f64, t5293: f64, t1261: f64, t20863: f64, t3172: f64, t20973: f64, t3647: f64, t21242: f64, t3636: f64, t17306: f64, t17728: f64, t489: f64) -> (f64, f64, f64, f64, f64) {
    let t69971 = t5293 * t17361;
    let t69984 = t1261 * t3172 * t20863;
    let t70006 = t3647 * t20973;
    let t70008 = t21242 * t3636;
    let t70014 = t17306 * t489 * t17728;
    (t69971, t69984, t70006, t70008, t70014)
}
