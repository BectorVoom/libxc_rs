//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 709/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk709(t7523: f64, t2595: f64, t56: f64, t214: f64, t136: f64, t2548: f64, t745: f64, t222: f64, t224: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7524 = 28.0_f64 / 27.0_f64 * t7523;
    let t7533 = t56 * t2595;
    let t7557 = 1.0_f64/pow_3_2(t214);
    let t7578 = t136 * t2548;
    let t7590 = t745 * t136;
    let t7592 = t222 * t7590 * t224;
    (t7524, t7533, t7557, t7578, t7590, t7592)
}
