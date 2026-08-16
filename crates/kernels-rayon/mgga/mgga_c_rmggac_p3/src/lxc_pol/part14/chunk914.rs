//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 914/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk914(t1971: f64, t2144: f64, t27136: f64, t3351: f64, t16043: f64, t9138: f64, t27120: f64, t875: f64, t27075: f64, t1657: f64, t2039: f64, t270: f64, t638: f64) -> (f64, f64, f64, f64, f64) {
    let t39771 = t3351 * t1971 * t2144 * t27136;
    let t39773 = t16043 * t9138;
    let t39777 = t3351 * t1971 * t875 * t27120;
    let t39781 = t3351 * t1971 * t875 * t27075;
    let t39785 = t638 * t2039 * t1657 * t270;
    (t39771, t39773, t39777, t39781, t39785)
}
