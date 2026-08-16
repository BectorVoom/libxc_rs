//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2699/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699(t1448: f64, t3829: f64, t3889: f64, t39989: f64, t4139: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t48305: f64, t48307: f64, t48308: f64, t48311: f64, t5542: f64) -> (f64, f64) {
    let t49616 = t3829 * t1448;
    let t49630 = t3889 * t1448;
    let t49634 = -9.0_f64 * t4139 * t49630 * t5542 - t39989 - t47086 + t47088 + t47092 - t47096 - t47098 + t48305 + t48307 + t48308 - t48311;
    (t49616, t49634)
}
