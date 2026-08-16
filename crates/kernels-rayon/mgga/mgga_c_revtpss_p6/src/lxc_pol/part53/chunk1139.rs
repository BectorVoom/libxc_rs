//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1139/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1139(t28025: f64, t7735: f64, t27137: f64, t6985: f64, t2322: f64, t33574: f64, t4254: f64, t651: f64, t7221: f64, t7741: f64, t25805: f64, t7742: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125541 = t28025 * t7735;
    let t125543 = t6985 * t27137;
    let t125545 = t2322 * t33574;
    let t125547 = t4254 * t33574;
    let t125550 = t651 * t7221 * t7741;
    let t125552 = t25805 * t7742;
    (t125541, t125543, t125545, t125547, t125550, t125552)
}
