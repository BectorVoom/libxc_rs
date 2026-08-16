//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 664/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk664(t289: f64, t8817: f64, t2408: f64, t275: f64, t1652: f64, t2060: f64, t739: f64, t2124: f64, t558: f64, t884: f64, t321: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8818 = t289 * t8817;
    let t8820 = t275 * t2408;
    let t8821 = t2060 * t1652;
    let t8822 = t739 * t8821;
    let t8824 = t2124 * t558;
    let t8825 = t884 * t8824;
    let t8829 = t615 * t321;
    (t8818, t8820, t8821, t8822, t8824, t8825, t8829)
}
