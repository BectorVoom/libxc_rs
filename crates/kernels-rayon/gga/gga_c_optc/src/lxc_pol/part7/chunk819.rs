//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 819/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk819(t2494: f64, t817: f64, t818: f64, t2493: f64, t2495: f64, t2518: f64, t252: f64, t2530: f64, t2537: f64, t7621: f64, t7626: f64, t7628: f64, t7631: f64, t7666: f64, t7688: f64, t7694: f64, t7710: f64, t7726: f64, t7727: f64, t7731: f64, t7734: f64, t7738: f64, t7741: f64, t7744: f64) -> (f64, f64, f64) {
    let t7747 = t2494 * t817;
    let t7748 = t7747 * t818;
    let t7752 = -0.3109e-1_f64 * t7710 * t252 - t7688 + t7694 + t7726 - 0.35089340384731224426e1_f64 * t2530 * t7727 + 0.51947267698127589897e2_f64 * t2537 * t7731 - 6.0_f64 * t2493 * t7734 + 0.96494049533612093922e2_f64 * t2518 * t7738 + 0.35089340384731224426e1_f64 * t2537 * t7741 - 6.0_f64 * t7744 * t2495 + 6.0_f64 * t2518 * t7748 - 0.19751789702565206229e-1_f64 * t7621 - t7626 - t7628 - t7631 - t7666;
    (t7747, t7748, t7752)
}
