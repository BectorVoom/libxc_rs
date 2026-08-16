//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1089/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1089(t125453: f64, t2014: f64, t32119: f64, t32297: f64, t5542: f64, t33657: f64, t7235: f64, t32114: f64, t7898: f64, t33652: f64, t22496: f64, t25082: f64, t37110: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125456 = 3.0_f64 * t2014 * t32119 * t125453;
    let t125470 = t2014 * t32297 * t5542;
    let t125472 = 3.0_f64 * t7235 * t33657;
    let t125474 = 2.0_f64 * t7898 * t32114;
    let t125483 = 2.0_f64 * t7235 * t33652;
    let t125486 = 6.0_f64 * t25082 * t37110 * t22496;
    (t125456, t125470, t125472, t125474, t125483, t125486)
}
