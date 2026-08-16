//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1254/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1254(t125584: f64, t120977: f64, t120983: f64, t122284: f64, t122288: f64, t122290: f64, t122297: f64, t122299: f64, t125590: f64, t1444: f64, t1903: f64, t32250: f64, t32673: f64, t34226: f64, t8706: f64) -> f64 {
    let t128595 = 0.13223814266738539448e-3_f64 * t125584;
    let t128609 = t128595 - 0.25702851531048074406e-1_f64 * t122284 - 0.14279934416275588154e-1_f64 * t122288 + 0.25389723392137995738e-1_f64 * t122290 - 0.17135921299530705785e1_f64 * t8706 * t32250 * t32673 * t1903 - 0.17135921299530705785e1_f64 * t8706 * t32250 * t34226 * t1444 + t122297 - 0.14279934416275588154e-1_f64 * t122299 - t120977 - 0.29749863367240808656e-2_f64 * t125590 - t120983;
    t128609
}
