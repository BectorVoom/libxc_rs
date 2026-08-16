//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1134/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1134(t28021: f64, t8568: f64, t8567: f64, t8995: f64, t28199: f64, t33652: f64, t7235: f64, t22496: f64, t25082: f64, t37110: f64, t2014: f64, t33975: f64, t7315: f64) -> (f64, f64, f64, f64, f64) {
    let t125475 = t8568 * t28021;
    let t125478 = t8567 * t8995;
    let t125479 = t125478 * t28199;
    let t125483 = 2.0_f64 * t7235 * t33652;
    let t125486 = 6.0_f64 * t25082 * t37110 * t22496;
    let t125488 = t2014 * t33975 * t7315;
    (t125475, t125479, t125483, t125486, t125488)
}
