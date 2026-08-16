//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1064/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1064(t11515: f64, t11520: f64, t12107: f64, t12108: f64, t12109: f64, t12110: f64, t12111: f64, t12112: f64, t12113: f64, t12114: f64, t12115: f64, t12116: f64, t12117: f64, t12118: f64, t12119: f64, t12120: f64, t12123: f64, t12124: f64, t12125: f64) -> f64 {
    let t12611 = t12107 - t12108 - t12109 - t12110 - t12111 + t12112 + t12113 - t12114 + t12115 + t12116 - t12117 + t12118 + t12119 - t12120 + 0.53968515702149165444e-6_f64 * t11515 + 0.49166375783284505216e-8_f64 * t11520 + t12123 + t12124 + t12125;
    t12611
}
