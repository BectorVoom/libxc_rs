//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1070/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1070(t3172: f64, t6307: f64, t3150: f64, t4820: f64, t4879: f64, t11725: f64, t247: f64, t6092: f64, t1063: f64, t3109: f64, t6100: f64, t1647: f64, t1678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20029 = t3172 * t6307;
    let t20030 = t3150 * t20029;
    let t20034 = t4879 * t4820;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    let t20055 = t1063 * t20054;
    let t20175 = t1647 * t1678;
    (t20029, t20030, t20034, t20050, t20051, t20054, t20055, t20175)
}
