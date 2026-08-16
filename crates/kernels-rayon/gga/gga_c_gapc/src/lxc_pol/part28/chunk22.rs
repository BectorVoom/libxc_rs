//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 22/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk22(t11: f64, t14: f64) -> (f64, f64, f64, f64) {
    let t65 = 0.107924e1_f64 + 0.3964e-1_f64 * t14 + 0.123825e-1_f64 * t11;
    let t68 = 1.0_f64 + t14 * t65 / 2.0_f64;
    let t69 = t68 * t68;
    let t70 = 1.0_f64 / t69;
    (t65, t68, t69, t70)
}
