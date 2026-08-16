//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1304/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1304(t1592: f64, t999: f64, t1045: f64, t15691: f64, t1066: f64, t18946: f64, t247: f64, t11725: f64, t6092: f64, t1063: f64, t3109: f64, t6100: f64) -> (f64, f64, f64, f64) {
    let t20038 = t1592 * t999;
    let t20039 = t1045 * t20038;
    let t20040 = t15691 * t20039;
    let t20046 = t247 * t1066 * t18946;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    (t20040, t20046, t20051, t20054)
}
