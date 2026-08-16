//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1302/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1302(t26252: f64, t26258: f64, t26262: f64, t26265: f64, t26268: f64, t26271: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26347: f64, t26351: f64, t26354: f64, t26358: f64) -> f64 {
    let t26360 = 0.44291358024691358024e0_f64 * t26252 + 0.39862222222222222223e1_f64 * t26258 + t26262 + t26265 + 0.1151859375e0_f64 * t26268 + 0.46074375e0_f64 * t26271 + 0.1898925e1_f64 * t26347 - 0.79724444444444444446e0_f64 * t26326 - 0.5314962962962962963e0_f64 * t26328 - 0.43816888888888888888e0_f64 * t26351 + 0.43816888888888888889e0_f64 * t26354 + 0.15944888888888888889e1_f64 * t26330 + 0.12401580246913580247e1_f64 * t26332 + 0.97370864197530864199e0_f64 * t26358;
    t26360
}
