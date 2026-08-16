//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 956/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk956(t31773: f64, t7452: f64, t7440: f64, t7444: f64, t7490: f64, t4680: f64, t7493: f64, t7642: f64, t1165: f64, t14575: f64, t604: f64, t7346: f64) -> (f64, f64, f64, f64, f64) {
    let t31774 = t31773 * t7452;
    let t31782 = t7440 * t7444;
    let t31790 = t7440 * t7490;
    let t31793 = t7493 * t4680 * t7642;
    let t31797 = t7346 * t1165 * t604 * t14575;
    (t31774, t31782, t31790, t31793, t31797)
}
