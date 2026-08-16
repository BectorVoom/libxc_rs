//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1214/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1214(t43212: f64, t52453: f64, t66197: f64, t66202: f64, t66221: f64, t80029: f64, t80031: f64, t88740: f64, t88744: f64, t88747: f64, t88751: f64, t88754: f64, t88761: f64, t88769: f64) -> f64 {
    let t91290 = 0.38514888888888888888e0_f64 * t80029 - 0.11554466666666666666e1_f64 * t80031 + 0.11554466666666666666e1_f64 * t88761 - 0.9628722222222222222e0_f64 * t88769 + 0.34663399999999999999e1_f64 * t88740 - 0.38514888888888888888e0_f64 * t88747 - 0.51995099999999999998e1_f64 * t88754 + 0.59912049382716049381e0_f64 * t52453 + t43212 - 0.25676592592592592592e0_f64 * t66197 - 0.38514888888888888888e0_f64 * t66202 + 0.77029777777777777776e0_f64 * t66221 - 0.28886166666666666666e0_f64 * t88744 + 0.34663399999999999999e1_f64 * t88751;
    t91290
}
