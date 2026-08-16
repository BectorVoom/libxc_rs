//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 351/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk351(t1576: f64, t1577: f64, t1487: f64, t465: f64, t1423: f64, t458: f64, t1338: f64, t437: f64) -> (f64, f64, f64, f64) {
    let t1578 = t1576 * t1577;
    let t1581 = t1487 * t465;
    let t1584 = t1423 * t458;
    let t1587 = t437 * t1338;
    (t1578, t1581, t1584, t1587)
}
