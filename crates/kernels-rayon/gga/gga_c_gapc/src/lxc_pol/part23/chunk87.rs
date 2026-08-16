//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 87/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk87(t62: f64, t80: f64, t85: f64, t88: f64, t97: f64) -> f64 {
    let t266 = -0.77371026992393176896e-2_f64 * t62 + 0.187495875e-2_f64 * t80 - 0.362780625e-3_f64 * t85 + 0.10208501871552144532e-4_f64 * t88 - 0.8659659375e-6_f64 * t97;
    t266
}
