//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 854/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk854(t3608: f64, t8164: f64, t2721: f64, t2725: f64, t2812: f64, t2814: f64, t8134: f64, t8135: f64, t8138: f64, t8140: f64, t8145: f64, t8149: f64, t8154: f64, t8157: f64, t8161: f64, t8165: f64, t8168: f64, t930: f64, t953: f64) -> (f64, f64) {
    let t8171 = t3608 * t8164;
    let t8174 = -0.8987985586528718635e4_f64 * t8134 * t8135 - 0.33587136305576131526e-2_f64 * t8138 - 0.12475836244235246496e3_f64 * t8140 * t2814 + 0.1559479530529405812e2_f64 * t8145 - 0.60587206808032502059e1_f64 * t8149 * t2725 + 0.75734008510040627575e0_f64 * t8154 + 0.11590881986385010473e0_f64 * t930 * t8157 + 0.25190352229182098644e-1_f64 * t953 * t8161 + 0.1949349413161757265e2_f64 * t2812 * t8165 + 0.11360101276506094136e1_f64 * t2721 * t8168 + 0.15146801702008125515e1_f64 * t2721 * t8171;
    (t8171, t8174)
}
