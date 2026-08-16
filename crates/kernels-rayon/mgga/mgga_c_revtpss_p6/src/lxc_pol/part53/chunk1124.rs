//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1124/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1124(t2172: f64, t7318: f64, t32910: f64, t571: f64, t1464: f64, t8766: f64, t2045: f64, t7690: f64, t2167: f64, t7337: f64, t4245: f64, t8453: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123122 = t7318 * t2172;
    let t123124 = t571 * t32910;
    let t123129 = t8766 * t1464;
    let t123131 = t7690 * t2045;
    let t123138 = t2167 * t7337;
    let t125209 = t4245 * t8453;
    (t123122, t123124, t123129, t123131, t123138, t125209)
}
