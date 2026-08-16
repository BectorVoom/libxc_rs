//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 783/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk783(t305: f64, t36293: f64, t674: f64, t7546: f64, t7715: f64, t20: f64, t2018: f64, t2021: f64, t4729: f64, t1969: f64, t8516: f64, t7229: f64, t7243: f64) -> (f64, f64, f64, f64, f64) {
    let t36294 = t305 * t36293;
    let t36315 = t7546 * t7715 * t674;
    let t36330 = t4729 * t20 * t2018 * t2021;
    let t36336 = t8516 * t1969;
    let t36343 = t7229 * t7243;
    (t36294, t36315, t36330, t36336, t36343)
}
