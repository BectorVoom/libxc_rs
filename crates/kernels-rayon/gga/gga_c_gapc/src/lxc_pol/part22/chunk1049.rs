//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1049/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1049(t11506: f64, t11510: f64, t11524: f64, t11527: f64, t11529: f64, t11515: f64, t11520: f64, t12107: f64, t12108: f64, t12109: f64, t12110: f64, t12111: f64, t12112: f64, t12113: f64, t12114: f64, t12115: f64, t12116: f64, t12117: f64, t12118: f64) -> f64 {
    let t12119 = 0.22510123728325872388e-6_f64 * t11506;
    let t12120 = 0.30353495895471971565e-6_f64 * t11510;
    let t12123 = 0.25301920572916666668e-5_f64 * t11524;
    let t12124 = 0.25301920572916666668e-5_f64 * t11527;
    let t12125 = 0.16217772716043213195e-2_f64 * t11529;
    let t12126 = t12107 - t12108 - t12109 - t12110 - t12111 + t12112 + t12113 - t12114 + t12115 + t12116 - t12117 + t12118 + t12119 - t12120 + 0.53968515702149165443e-6_f64 * t11515 + 0.49166375783284505217e-8_f64 * t11520 + t12123 + t12124 + t12125;
    t12126
}
