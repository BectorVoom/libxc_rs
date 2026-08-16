//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 422/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk422(t358: f64, t942: f64, t363: f64, t1564: f64, t446: f64, t1586: f64, t432: f64, t28: f64, t89: f64, t1597: f64, t383: f64, t1594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3008 = t942 * t358;
    let t3009 = t3008 * t363;
    let t3010 = t1564 * t3009;
    let t3011 = t446 * t3010;
    let t3013 = t1586 * t942;
    let t3014 = t3013 * t432;
    let t3016 = t89 * t28 * t3014;
    let t3018 = t383 * t1597;
    let t3019 = t1594 * t3018;
    (t3009, t3010, t3011, t3013, t3014, t3016, t3019)
}
