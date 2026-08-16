//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 172/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk172(t118: f64, t571: f64, t553: f64, t560: f64) -> (f64, f64) {
    let t572 = t118 * t571;
    let t574 = -0.59871208509319042821e-1_f64 * t553 + 0.59871208509319042821e-1_f64 * t560 + 0.19957069503106347607e-1_f64 * t572;
    (t572, t574)
}
