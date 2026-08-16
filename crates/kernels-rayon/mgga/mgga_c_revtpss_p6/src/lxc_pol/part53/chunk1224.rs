//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1224/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1224(t28043: f64, t7586: f64, t28056: f64, t125556: f64, t125558: f64, t125562: f64, t125566: f64, t27060: f64, t27145: f64, t28053: f64, t29427: f64, t29432: f64, t32825: f64, t32869: f64, t4248: f64, t4293: f64, t7007: f64, t7746: f64) -> f64 {
    let t129395 = t7586 * t28043;
    let t129407 = t7586 * t28056;
    let t129411 = -2.0_f64 * t27060 * t7746 - 2.0_f64 * t27145 * t7586 - 2.0_f64 * t28053 * t7586 - 2.0_f64 * t29427 * t7007 - 2.0_f64 * t29432 * t7746 - 2.0_f64 * t32825 * t4293 - 2.0_f64 * t32869 * t4248 - 2.0_f64 * t125556 + t125558 - t125562 + t125566 - 2.0_f64 * t129395 - 2.0_f64 * t129407;
    t129411
}
