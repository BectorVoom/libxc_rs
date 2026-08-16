//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1028/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1028(t4295: f64, t7822: f64, t4300: f64, t4304: f64, t30374: f64, t8657: f64, t30811: f64, t4904: f64, t2450: f64, t7431: f64, t8461: f64, t8653: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34150 = t7822 * t4295;
    let t34152 = t7822 * t4300;
    let t34154 = t7822 * t4304;
    let t34156 = t30374 * t8657;
    let t34158 = t30811 * t4904;
    let t34159 = 0.68598428988911579156e-2_f64 * t34158;
    let t34161 = t2450 * t7431 * t8461;
    let t34162 = t34161 * t8653;
    (t34150, t34152, t34154, t34156, t34159, t34161, t34162)
}
