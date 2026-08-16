//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk817(t7523: f64, t7525: f64, t7527: f64, t7529: f64, t7531: f64, t7535: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7550: f64, t232: f64) -> (f64, f64) {
    let t7713 = 0.55403703703703703703e-1_f64 * t7523;
    let t7724 = -t7713 - 0.23744444444444444444e-1_f64 * t7525 + 0.11872222222222222222e-1_f64 * t7527 - 0.35616666666666666666e-1_f64 * t7529 + 0.17808333333333333333e-1_f64 * t7531 - 0.19787037037037037037e-1_f64 * t7535 + 0.71233333333333333332e-1_f64 * t7538 - 0.35616666666666666666e-1_f64 * t7541 - 0.10685e0_f64 * t7544 + 0.10685e0_f64 * t7547 - 0.17808333333333333333e-1_f64 * t7550;
    let t7726 = 0.62182e-1_f64 * t7724 * t232;
    (t7724, t7726)
}
