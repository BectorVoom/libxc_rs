//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 333/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk333(t1431: f64, t527: f64, t436: f64, t1: f64, t432: f64, t463: f64, t468: f64, t584: f64, t624: f64, t474: f64, t505: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1432 = t527 * t1431;
    let t1433 = t436 * t1432;
    let t1436 = t432 * t1;
    let t1437 = t463 * t1436;
    let t1438 = t468 * t584;
    let t1441 = t468 * t624;
    let t1444 = t474 * t505;
    let t1445 = t1444 * t476;
    (t1432, t1433, t1437, t1438, t1441, t1444, t1445)
}
