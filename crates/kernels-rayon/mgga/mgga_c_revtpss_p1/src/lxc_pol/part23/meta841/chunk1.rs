//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2717/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717(t21101: f64, t3707: f64, t17608: f64, t5292: f64, t17547: f64, t5265: f64, t1261: f64, t20906: f64, t3172: f64, t17416: f64, t5391: f64, t21272: f64, t3636: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70082 = t3707 * t21101;
    let t70088 = t17608 * t5292;
    let t70091 = t17547 * t5265;
    let t70102 = t1261 * t3172 * t20906;
    let t70112 = t5391 * t17416;
    let t70114 = t21272 * t3636;
    (t70082, t70088, t70091, t70102, t70112, t70114)
}
