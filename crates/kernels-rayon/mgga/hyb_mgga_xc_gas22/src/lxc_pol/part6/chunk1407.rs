//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1407/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1407(t21990: f64, t21994: f64, t21997: f64, t21999: f64, t22004: f64, t22009: f64, t22010: f64, t22012: f64, t22014: f64, t22019: f64, t22024: f64, t22030: f64, t22033: f64, t22038: f64, t22042: f64, t22045: f64, t22046: f64, t30373: f64) -> f64 {
    let t30439 = t21990 - t21994 - 24.0_f64 * t21997 - 0.20508037716432813316e4_f64 * t21999 - t30373 - 240.0_f64 * t22004 + t22009 + 0.70178683471615754484e1_f64 * t22010 - 0.11393789434848516922e-2_f64 * t22012 - 0.10389515463408878255e3_f64 * t22014 - t22019 - t22024 - t22030 - t22033 - t22038 - t22042 - t22045 + 0.24415263074675393405e-3_f64 * t22046;
    t30439
}
