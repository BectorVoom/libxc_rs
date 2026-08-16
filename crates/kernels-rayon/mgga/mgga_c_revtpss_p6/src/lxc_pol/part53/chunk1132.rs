//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1132/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1132(t33591: f64, t4254: f64, t1936: f64, t27830: f64, t651: f64, t1937: f64, t97622: f64, t108120: f64, t28030: f64, t6993: f64, t4147: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125433 = t4254 * t33591;
    let t125436 = t651 * t27830 * t1936;
    let t125438 = t97622 * t1937;
    let t125442 = t108120 * t1937;
    let t125444 = t28030 * t6993;
    let t125453 = t4147 * t5591;
    (t125433, t125436, t125438, t125442, t125444, t125453)
}
