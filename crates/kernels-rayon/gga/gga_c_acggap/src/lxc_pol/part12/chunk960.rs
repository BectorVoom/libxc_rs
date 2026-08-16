//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 960/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk960(t2109: f64, t7780: f64, t1980: f64, t31028: f64, t7476: f64, t1988: f64, t7701: f64, t7705: f64, t1982: f64, t2015: f64, t7452: f64, t7440: f64, t7444: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31752 = t7780 * t2109;
    let t31759 = t1980 * t7476 * t31028;
    let t31761 = t1988 * t7701;
    let t31763 = t1988 * t7705;
    let t31773 = t2015 * t1982;
    let t31774 = t31773 * t7452;
    let t31782 = t7440 * t7444;
    (t31752, t31759, t31761, t31763, t31773, t31774, t31782)
}
