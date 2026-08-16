//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2661/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661(t1647: f64, t16565: f64, t12166: f64, t1678: f64, t342: f64, t12077: f64, t20050: f64, t3106: f64, t1063: f64, t247: f64, t42447: f64, t6092: f64) -> (f64, f64, f64, f64, f64) {
    let t65181 = t1647 * t16565;
    let t65216 = t342 * t12166 * t1678;
    let t65220 = t342 * t12077 * t1678;
    let t65288 = t3106 * t20050;
    let t65292 = t1063 * t247 * t42447 * t6092;
    (t65181, t65216, t65220, t65288, t65292)
}
