//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 652/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk652(t739: f64, t9321: f64, t1587: f64, t699: f64, t1652: f64, t2211: f64, t1356: f64, t1664: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9322 = t739 * t9321;
    let t9332 = t699 * t1587;
    let t9333 = t739 * t9332;
    let t9340 = t2211 * t1652;
    let t9341 = t1356 * t9340;
    let t9343 = t1664 * t702;
    (t9322, t9332, t9333, t9340, t9341, t9343)
}
