//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 951/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk951(t1882: f64, t20912: f64, t20894: f64, t20945: f64, t20729: f64, t20733: f64, t20711: f64, t20909: f64, t20972: f64, t604: f64, t20702: f64, t20851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77481 = t1882 * t20912;
    let t77487 = t1882 * t20894;
    let t77489 = t1882 * t20945;
    let t77491 = t1882 * t20729;
    let t77505 = t1882 * t20733;
    let t77521 = t1882 * t20711;
    let t77575 = t1882 * t20909;
    let t77602 = t604 * t20972;
    let t77610 = t1882 * t20702;
    let t77633 = t20851 * t604;
    (t77481, t77487, t77489, t77491, t77505, t77521, t77575, t77602, t77610, t77633)
}
