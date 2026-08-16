//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 950/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk950(t3037: f64, t3922: f64, t449: f64, t556: f64, t1308: f64, t3889: f64, t545: f64, t848: f64, t464: f64, t1219: f64, t1658: f64, t5384: f64, t871: f64) -> (f64, f64, f64, f64, f64) {
    let t15110 = t3922 * t449 * t556 * t3037;
    let t15112 = t1308 * t3889;
    let t15115 = t848 * t545;
    let t15116 = t15115 * t464;
    let t15126 = t5384 * t1219 * t1658 * t871;
    (t15110, t15112, t15115, t15116, t15126)
}
