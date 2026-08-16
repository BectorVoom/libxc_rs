//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1300/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1300(t109173: f64, t2014: f64, t7900: f64, t1583: f64, t5966: f64, t25207: f64, t23279: f64, t27159: f64, t1544: f64, t6075: f64, t27383: f64, t1468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t113095 = 9.0_f64 * t2014 * t109173 * t7900;
    let t113096 = t5966 * t1583;
    let t113097 = t25207 * t113096;
    let t113100 = t27159 * t23279;
    let t113103 = t1544 * t6075;
    let t113104 = t25207 * t113103;
    let t113107 = t1583 * t6075;
    let t113108 = t27383 * t113107;
    let t113111 = t1468 * t6075;
    (t113095, t113096, t113097, t113100, t113103, t113104, t113107, t113108, t113111)
}
