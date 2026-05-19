//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1049/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1049<F: Float>(t11506: F, t11510: F, t11524: F, t11527: F, t11529: F, t11515: F, t11520: F, t12107: F, t12108: F, t12109: F, t12110: F, t12111: F, t12112: F, t12113: F, t12114: F, t12115: F, t12116: F, t12117: F, t12118: F) -> F {
    let t12119 = F::cast_from(0.22510123728325872388e-6_f64) * t11506;
    let t12120 = F::cast_from(0.30353495895471971565e-6_f64) * t11510;
    let t12123 = F::cast_from(0.25301920572916666668e-5_f64) * t11524;
    let t12124 = F::cast_from(0.25301920572916666668e-5_f64) * t11527;
    let t12125 = F::cast_from(0.16217772716043213195e-2_f64) * t11529;
    let t12126 = t12107 - t12108 - t12109 - t12110 - t12111 + t12112 + t12113 - t12114 + t12115 + t12116 - t12117 + t12118 + t12119 - t12120 + F::cast_from(0.53968515702149165443e-6_f64) * t11515 + F::cast_from(0.49166375783284505217e-8_f64) * t11520 + t12123 + t12124 + t12125;
    t12126
}
