//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 29/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk29(t43: f64, t47: f64, t46: f64, t42: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t48 = t47 * t43;
    let t49 = piecewise3(t44, t46, t48);
    let t50 = 1.0_f64 - t42;
    (t48, t49, t50)
}
