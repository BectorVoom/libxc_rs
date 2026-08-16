//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 816/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk816(t2373: f64, t7692: f64, t2472: f64, t7342: f64, t837: f64, t845: f64, t7523: f64, t7525: f64, t7527: f64, t7529: f64, t7531: f64, t7535: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7550: f64) -> (f64, f64, f64, f64) {
    let t7694 = 6.0_f64 * t2373 * t7692;
    let t7696 = t2472 * t7342 * t837;
    let t7698 = 0.35089340384731224426e1_f64 * t845 * t7696;
    let t7699 = 0.53272592592592592592e-1_f64 * t7523;
    let t7710 = -t7699 - 0.2283111111111111111e-1_f64 * t7525 + 0.11415555555555555555e-1_f64 * t7527 - 0.34246666666666666665e-1_f64 * t7529 + 0.17123333333333333333e-1_f64 * t7531 - 0.19025925925925925925e-1_f64 * t7535 + 0.68493333333333333331e-1_f64 * t7538 - 0.34246666666666666665e-1_f64 * t7541 - 0.10274e0_f64 * t7544 + 0.10274e0_f64 * t7547 - 0.17123333333333333333e-1_f64 * t7550;
    (t7694, t7696, t7698, t7710)
}
