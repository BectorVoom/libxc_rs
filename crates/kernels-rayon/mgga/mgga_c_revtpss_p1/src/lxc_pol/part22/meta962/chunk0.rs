//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3224/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3224(t2609: f64, t2611: f64, t5819: f64, t49957: f64, t49963: f64, t49966: f64, t49978: f64, t49981: f64, t49983: f64, t49986: f64, t39779: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61165 = t2611 * t2609 * t5819;
    let t61166 = 12.0_f64 * t61165;
    let t61167 = 0.23392894490538584828e1_f64 * t49957;
    let t61168 = 0.69263436422725855034e2_f64 * t49963;
    let t61169 = 0.11696447245269292414e1_f64 * t49966;
    let t61170 = 8.0_f64 * t49978;
    let t61171 = 16.0_f64 * t49981;
    let t61172 = 8.0_f64 * t49983;
    let t61173 = 0.36622894612013090108e-3_f64 * t49986;
    let t61174 = t61166 - t61167 + t39779 - t61168 - t61169 - t39783 - t39786 - t39791 - t39795 + t61170 + t61171 + t61172 - t61173;
    (t61166, t61167, t61168, t61169, t61170, t61171, t61172, t61173, t61174)
}
