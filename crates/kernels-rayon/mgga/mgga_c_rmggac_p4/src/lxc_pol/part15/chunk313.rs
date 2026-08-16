//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 313/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk313(t1174: f64, t1459: f64, t1857: f64, t1860: f64, t1864: f64, t1907: f64, t228: f64, t462: f64, t598: f64) -> f64 {
    let t1910 = t1857 * t228 + t1860 * t228 + t598 * t1459 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t1174 * t1864 + t462 * t1907 / 4.0_f64;
    t1910
}
