//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1161/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1161(t2264: f64, t24128: f64, t7222: f64, t7218: f64, t7307: f64, t2441: f64, t7318: f64, t2529: f64, t7604: f64, t838: f64, t845: f64, t23: f64, t2326: f64, t2328: f64, t2331: f64) -> (f64, f64, f64, f64, f64) {
    let t24130 = t24128 * t7222 * t2264;
    let t24133 = t7307 * t7218;
    let t24137 = 0.2077890707925103596e3_f64 * t2441 * t7318;
    let t24141 = 0.46785787179641632568e1_f64 * t845 * t2529 * t7604 * t838;
    let t24145 = t2326 * t2328 * t2331 * t23;
    (t24130, t24133, t24137, t24141, t24145)
}
