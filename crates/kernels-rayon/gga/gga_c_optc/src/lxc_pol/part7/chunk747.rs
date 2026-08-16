//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 747/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk747(t115: f64, t5: f64, t7192: f64, t363: f64, t2343: f64, t992: f64, t355: f64, t2529: f64, t2534: f64, t836: f64, t845: f64, t529: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7194 = t7192 * t115 * t5;
    let t7195 = t7194 * t363;
    let t7198 = t2343 * t992;
    let t7199 = t355 * t7198;
    let t7202 = t2529 * t836 * t2534;
    let t7204 = 0.35089340384731224426e1_f64 * t845 * t7202;
    let t7205 = t6 * t529;
    (t7194, t7195, t7198, t7199, t7202, t7204, t7205)
}
