//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 22/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk22(t45: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46 = t45 / 2.0_f64;
    let t47 = pow_1_3(t46);
    let t48 = t47 * t47;
    let t49 = t48 * t46;
    let t51 = rho1 * rho1;
    let t52 = pow_1_3(rho1);
    let t53 = t52 * t52;
    let t55 = 1.0_f64 / t53 / t51;
    (t46, t47, t48, t49, t51, t52, t53, t55)
}
