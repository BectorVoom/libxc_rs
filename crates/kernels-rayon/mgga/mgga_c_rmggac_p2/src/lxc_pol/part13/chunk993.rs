//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 993/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk993(t3351: f64, t352: f64, t515: f64, t9210: f64, t9211: f64, t6355: f64, t7707: f64, t1550: f64, t41548: f64, t34975: f64, t34976: f64, t7455: f64, t8440: f64) -> (f64, f64, f64, f64) {
    let t41784 = t3351 * t9210 * t515 * t9211 * t352;
    let t41789 = t6355 * t7707;
    let t41791 = t1550 * t41548;
    let t41796 = t34975 * t34976 * t8440 * t7455;
    (t41784, t41789, t41791, t41796)
}
