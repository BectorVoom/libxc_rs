//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 106/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk106(t216: f64, t232: f64, t46: f64, t56: f64, t59: f64, zeta_threshold: f64) -> (f64, f64) {
    let t234 = 0.62182e-1_f64 * t216 * t232;
    let t235 = 2.0_f64 <= zeta_threshold;
    let t237 = piecewise3(t235, t46, 2.0_f64 * t56);
    let t238 = 0.0_f64 <= zeta_threshold;
    let t239 = piecewise3(t238, t46, 0.0_f64);
    let t241 = (t237 + t239 - 2.0_f64) * t59;
    (t234, t241)
}
