//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1306/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1306(t11860: f64, t19501: f64, t3117: f64, t19611: f64, t3095: f64, t3092: f64, t19414: f64, t247: f64, t3116: f64, t1651: f64, t4866: f64, t1045: f64) -> (f64, f64, f64, f64, f64) {
    let t20074 = t19501 * t11860;
    let t20075 = t3117 * t20074;
    let t20078 = t19611 * t3095;
    let t20079 = t3092 * t20078;
    let t20083 = t247 * t3116 * t19414;
    let t20089 = t1651 * t4866;
    let t20090 = t20089 * t1045;
    (t20075, t20079, t20083, t20089, t20090)
}
