//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 488/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk488<F: Float>(t2723: F, t298: F, t181: F, t604: F, t892: F, t1636: F, t291: F, t906: F, t2404: F, t966: F, t330: F, t197: F) -> (F, F, F, F) {
    let t2724 = t298 * t2723;
    let t2725 = t181 * t2724;
    let t2728 = t604 * t892;
    let t2732 = t1636 * t291 * t906;
    let t2735 = t966 * t2404;
    let t2736 = t330 * t2735;
    let t2737 = t197 * t2736;
    (t2725, t2728, t2732, t2737)
}
