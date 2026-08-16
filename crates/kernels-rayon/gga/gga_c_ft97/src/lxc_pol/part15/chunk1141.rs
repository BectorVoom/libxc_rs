//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1141/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1141(t18159: f64, t193: f64, t5053: f64, t89: f64, t41966: f64, t88252: f64, t9716: f64, t666: f64, t9749: f64, t1131: f64, t80748: f64, t1091: f64, t21477: f64) -> (f64, f64, f64, f64, f64) {
    let t89069 = t89 * t193 * t18159 * t5053;
    let t89073 = t89 * t9716 * t41966 * t88252;
    let t89077 = t89 * t666 * t9749 * t88252;
    let t89081 = t89 * t193 * t80748 * t1131;
    let t89083 = t1091 * t21477;
    (t89069, t89073, t89077, t89081, t89083)
}
