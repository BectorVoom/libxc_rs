//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2550/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2550(t1045: f64, t20089: f64, t3117: f64, t1651: f64, t2857: f64, t4181: f64, t3092: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20090 = t20089 * t1045;
    let t20091 = t3117 * t20090;
    let t20094 = t1651 * t2857;
    let t20095 = t20094 * t4181;
    let t20096 = t3092 * t20095;
    let t20099 = t1651 * t2852;
    (t20090, t20091, t20094, t20095, t20096, t20099)
}
