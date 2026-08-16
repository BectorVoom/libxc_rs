//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1216/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1216(t125420: f64, t125431: f64, t125432: f64, t125433: f64, t125436: f64, t125438: f64, t125442: f64, t125444: f64, t125456: f64, t125459: f64, t125467: f64, t2127: f64, t27830: f64, t7584: f64, t7883: f64) -> f64 {
    let t129298 = -t2127 * t27830 - t7584 * t7883 - 2.0_f64 * t125420 - t125431 - t125432 - 2.0_f64 * t125433 - 2.0_f64 * t125436 - 2.0_f64 * t125438 - 2.0_f64 * t125442 - 2.0_f64 * t125444 - t125456 - 2.0_f64 * t125459 - t125467;
    t129298
}
