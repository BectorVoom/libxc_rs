//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 731/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk731(t7698: f64, t1089: f64, t2090: f64, t3201: f64, t598: f64, t1083: f64, t7533: f64, t1459: f64, t7458: f64, t7486: f64, t1980: f64, t2117: f64, t377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7699 = 0.42874018118069736972e-3_f64 * t7698;
    let t7701 = t1089 * t3201 * t2090;
    let t7702 = t598 * t7701;
    let t7705 = t1089 * t1083 * t7533;
    let t7706 = t598 * t7705;
    let t7709 = t7458 * t1459 * t7486;
    let t7710 = t1980 * t7709;
    let t7712 = t377 * t2117;
    (t7699, t7701, t7702, t7705, t7706, t7709, t7710, t7712)
}
