//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 854/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk854<F: Float>(t3608: F, t8164: F, t2721: F, t2725: F, t2812: F, t2814: F, t8134: F, t8135: F, t8138: F, t8140: F, t8145: F, t8149: F, t8154: F, t8157: F, t8161: F, t8165: F, t8168: F, t930: F, t953: F) -> (F, F) {
    let t8171 = t3608 * t8164;
    let t8174 = -F::cast_from(0.8987985586528718635e4_f64) * t8134 * t8135 - F::cast_from(0.33587136305576131526e-2_f64) * t8138 - F::cast_from(0.12475836244235246496e3_f64) * t8140 * t2814 + F::cast_from(0.1559479530529405812e2_f64) * t8145 - F::cast_from(0.60587206808032502059e1_f64) * t8149 * t2725 + F::cast_from(0.75734008510040627575e0_f64) * t8154 + F::cast_from(0.11590881986385010473e0_f64) * t930 * t8157 + F::cast_from(0.25190352229182098644e-1_f64) * t953 * t8161 + F::cast_from(0.1949349413161757265e2_f64) * t2812 * t8165 + F::cast_from(0.11360101276506094136e1_f64) * t2721 * t8168 + F::cast_from(0.15146801702008125515e1_f64) * t2721 * t8171;
    (t8171, t8174)
}
