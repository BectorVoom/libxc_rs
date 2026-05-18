//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 329/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk329<F: Float>(t1431: F, t527: F, t436: F, t1: F, t432: F, t463: F, t468: F, t584: F, t624: F, t474: F, t505: F, t476: F) -> (F, F, F, F, F, F, F) {
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
