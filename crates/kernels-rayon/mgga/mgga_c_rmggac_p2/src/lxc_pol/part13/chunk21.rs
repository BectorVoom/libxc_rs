//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 21/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk21(t41: f64, t21: f64, t17: f64, t28: f64) -> (f64, f64, f64, f64) {
    let t43 = 1.0_f64 - 1.0_f64 / t41;
    let t45 = t21 * t43 + 1.0_f64;
    let t46 = f64::ln(t45);
    let t48 = -0.285764e-1_f64 * t17 + 0.285764e-1_f64 * t46;
    let t49 = t28 - 1.0_f64;
    (t43, t45, t48, t49)
}
