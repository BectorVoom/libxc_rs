//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 266/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk266(t1163: f64, t1166: f64, t1168: f64, t1174: f64, t1175: f64, t1240: f64, t228: f64, t458: f64, t462: f64) -> f64 {
    let t1243 = t1163 * t228 + t1166 * t228 + t458 * t1168 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t1174 * t1175 + t462 * t1240 / 4.0_f64;
    t1243
}
