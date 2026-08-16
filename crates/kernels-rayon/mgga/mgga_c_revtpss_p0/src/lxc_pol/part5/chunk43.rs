//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 43/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk43(t106: f64, t108: f64, t101: f64, t105: f64, t97: f64, t69: f64) -> (f64, f64, f64, f64, f64) {
    let t109 = t108 * t106;
    let t111 = t101 * t97 + t105 * t109;
    let t112 = 1.0_f64 / t111;
    let t114 = t69 * t112 / 8.0_f64;
    let t115 = 1.0_f64 < t114;
    let t116 = piecewise3(t115, 1.0_f64, t114);
    (t109, t111, t112, t116, t114)
}
