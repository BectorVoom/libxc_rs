//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1126/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1126(t125614: f64, t125673: f64, t125712: f64, t125763: f64, t125816: f64, t125857: f64, t125894: f64, t125932: f64, t1450: f64, t2014: f64, t532: f64, t1448: f64, t7933: f64) -> (f64, f64) {
    let t125938 = t2014 * t532 * (t125614 + t125673 + t125712 + t125763 + t125816 + t125857 + t125894 + t125932) * t1450;
    let t125939 = t7933 * t1448;
    (t125938, t125939)
}
