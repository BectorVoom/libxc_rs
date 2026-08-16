//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1113/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1113(t119833: f64, t121173: f64, t124: f64, t1426: f64, t13847: f64, t1444: f64, t25898: f64, t786: f64, t8578: f64, t4104: f64, t32699: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121174 = t119833 * t121173;
    let t121175 = t124 * t1426;
    let t121177 = t13847 * t121175 * t1444;
    let t121178 = t121174 * t121177;
    let t121181 = t786 * t8578 * t25898;
    let t121182 = t121181 * t4104;
    let t121184 = t32699 * t4075;
    (t121174, t121175, t121177, t121178, t121181, t121182, t121184)
}
