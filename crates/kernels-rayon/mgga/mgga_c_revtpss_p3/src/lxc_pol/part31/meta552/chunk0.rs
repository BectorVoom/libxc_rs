//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1954/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1954(t114: f64, t7898: f64, t7937: f64, t5542: f64, t7934: f64, t2014: f64, t25826: f64, t5891: f64, t5915: f64, t6998: f64, t25822: f64, t28679: f64) -> (f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t29993 = 2.0_f64 * t7898 * t7937;
    let t29996 = t7934 * t5542;
    let t29998 = 2.0_f64 * t2014 * t29996;
    let t29999 = t25826 * t5891;
    let t30001 = t6998 * t5915;
    let t30004 = piecewise3(t115, 0.0_f64, t25822 + t28679 + t29999 / 4.0_f64 - t30001 / 8.0_f64);
    (t29993, t29996, t29998, t30004)
}
