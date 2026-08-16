//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1094/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1094(t1936: f64, t27830: f64, t651: f64, t1937: f64, t97622: f64, t119535: f64, t125350: f64, t125402: f64, t125405: f64, t125407: f64, t125409: f64, t125410: f64, t125415: f64, t125417: f64, t125420: f64, t125431: f64, t125432: f64, t125433: f64, t1502: f64, t1519: f64, t32095: f64, t32162: f64, t4257: f64, t4297: f64) -> f64 {
    let t125436 = t651 * t27830 * t1936;
    let t125438 = t97622 * t1937;
    let t125440 = -2.0_f64 * t119535 * t1519 - 2.0_f64 * t125350 * t1519 - t1502 * t32095 - 2.0_f64 * t32162 * t4257 - 2.0_f64 * t32162 * t4297 + 6.0_f64 * t125402 - t125405 - t125407 - t125409 + 6.0_f64 * t125410 + t125415 - t125417 - 4.0_f64 * t125420 - t125431 - t125432 - 4.0_f64 * t125433 - 4.0_f64 * t125436 - 4.0_f64 * t125438;
    t125440
}
