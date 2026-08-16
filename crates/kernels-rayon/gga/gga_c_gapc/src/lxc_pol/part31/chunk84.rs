//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 84/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk84(t179: f64, t182: f64, t192: f64, t205: f64, t62: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t208 = 1.0_f64 + 0.13900948042322754167e-2_f64 * t179 * t182 - 0.57970906942607043474e-5_f64 * t192 * t205;
    let t209 = 1.0_f64 / t208;
    let t211 = rho0 - rho1;
    let t212 = t211 * t62;
    let t213 = 1.0_f64 + t212;
    let t214 = t213 <= zeta_threshold;
    let t215 = pow_1_3(t213);
    (t208, t209, t211, t212, t213, t215)
}
