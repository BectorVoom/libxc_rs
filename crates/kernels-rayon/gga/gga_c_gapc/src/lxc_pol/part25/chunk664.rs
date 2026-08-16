//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 664/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk664(t1431: f64, t442: f64, t128: f64, t1631: f64, t1463: f64, t1457: f64, t431: f64) -> (f64, f64, f64, f64) {
    let t4687 = t1431 * t442;
    let t4780 = t1631 * t128;
    let t4855 = t1463 * t442;
    let t4864 = t431 * t1457;
    (t4687, t4780, t4855, t4864)
}
