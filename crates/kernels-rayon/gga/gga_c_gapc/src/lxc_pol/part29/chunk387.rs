//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 387/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk387(t1509: f64, t1839: f64, t201: f64, t197: f64, t423: f64, t431: f64) -> (f64, f64) {
    let t1840 = t1839 * t1509;
    let t1841 = t201 * t1840;
    let t1842 = t197 * t1841;
    let t1845 = t423 * t431;
    (t1842, t1845)
}
