//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 963/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk963(t2155: f64, t30005: f64, t1222: f64, t7973: f64, t309: f64, t945: f64, t7963: f64, t9033: f64, t1221: f64, t2138: f64, t2139: f64, t8004: f64) -> (f64, f64, f64, f64) {
    let t31926 = t30005 * t2155;
    let t31928 = t7973 * t1222;
    let t31935 = t945 * t309;
    let t31937 = t7963 * t9033 * t31935;
    let t31944 = t2138 * t8004 * t2139 * t1221;
    (t31926, t31928, t31937, t31944)
}
