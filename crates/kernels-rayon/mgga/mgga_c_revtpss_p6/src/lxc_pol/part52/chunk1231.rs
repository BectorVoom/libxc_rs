//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1231/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1231(t34270: f64, t7316: f64, t2033: f64, t25082: f64, t26405: f64, t5591: f64, t125559: f64, t2014: f64, t32113: f64, t8108: f64, t34021: f64, t7235: f64) -> (f64, f64, f64, f64, f64) {
    let t128245 = t34270 * t7316;
    let t128251 = 3.0_f64 * t25082 * t26405 * t2033 * t5591;
    let t128254 = 3.0_f64 * t25082 * t26405 * t125559;
    let t128256 = t2014 * t8108 * t32113;
    let t128260 = 3.0_f64 * t7235 * t34021;
    (t128245, t128251, t128254, t128256, t128260)
}
