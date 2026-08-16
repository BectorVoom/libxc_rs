//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 736/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk736(t235: f64, t9731: f64, t1389: f64, t3964: f64, t2735: f64, t546: f64, t1369: f64, t2699: f64, t3943: f64, t794: f64, t1412: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9732 = t235 * t9731;
    let t9735 = 0.81322168495418382223e-4_f64 * t3964 * t9732 * t1389;
    let t9736 = t2735 * t546;
    let t9741 = t2699 * t1369;
    let t9744 = t794 * t3943;
    let t9747 = t159 * t1412;
    (t9732, t9735, t9736, t9741, t9744, t9747)
}
