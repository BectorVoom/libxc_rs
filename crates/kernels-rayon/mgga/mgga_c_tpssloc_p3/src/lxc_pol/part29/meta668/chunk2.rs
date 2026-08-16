//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2232/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2232(t16153: f64, t221: f64, t26284: f64, t26289: f64, t6604: f64, t80887: f64, t91133: f64, t91136: f64, t91138: f64, t91141: f64, t91144: f64, t91145: f64, t91147: f64, t91149: f64, t91155: f64, t91159: f64, t91162: f64, t91163: f64, t91165: f64, t91167: f64, t91171: f64, t91173: f64) -> f64 {
    let t91176 = t26284 * t221 * t16153;
    let t91179 = t80887 * t6604 * t26289;
    let t91180 = 0.11869590291677274911e0_f64 * t91179;
    let t91181 = 5.0_f64 / 384.0_f64 * t91133 + t91136 + t91138 - t91141 - t91144 - t91145 / 192.0_f64 - t91147 / 384.0_f64 - 119.0_f64 / 1728.0_f64 * t91149 + t91155 - t91159 + t91162 - t91163 / 384.0_f64 - t91165 / 384.0_f64 - 0.11304371706359309439e-1_f64 * t91167 - t91171 + t91173 / 8.0_f64 + t91176 / 16.0_f64 - t91180;
    t91181
}
