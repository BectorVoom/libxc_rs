//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1095/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1095(t1393: f64, t1457: f64, t1303: f64, t1463: f64, t1388: f64, t1044: f64, t128: f64, t188: f64, t1386: f64, t1642: f64, t5963: f64, t5973: f64, t662: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21655 = t1393 * t1457;
    let t21657 = t1463 * t1303;
    let t21778 = t1388 * t1457;
    let t21801 = t1044 * t188 * t128;
    let t21825 = t1386 * t1642;
    let t21838 = t5963 * t662 * t5973;
    (t21655, t21657, t21778, t21801, t21825, t21838)
}
