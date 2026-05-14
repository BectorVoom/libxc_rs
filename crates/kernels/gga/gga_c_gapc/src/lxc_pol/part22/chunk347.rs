//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 347/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk347<F: Float>(t1587: F, t436: F, t464: F, t640: F, t463: F, t1: F, t203: F, t3: F, t567: F, t1417: F, t126: F, t516: F, t514: F, t19: F, t147: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1588 = t436 * t1587;
    let t1591 = t464 * t640;
    let t1592 = t463 * t1591;
    let t1593 = t203 * t1;
    let t1595 = t1593 * t3 * t567;
    let t1596 = t1417 * t1595;
    let t1599 = t516 * t126;
    let t1600 = t514 * t1599;
    let t1601 = t203 * t19;
    let t1602 = t147 * t567;
    let t1603 = t1601 * t1602;
    (t1588, t1592, t1593, t1595, t1596, t1599, t1600, t1601, t1602, t1603)
}
