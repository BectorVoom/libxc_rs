//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 769/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk769<F: Float>(t1150: F, t12597: F, t1119: F, t141: F, t3233: F, t11325: F, t4456: F, t4463: F, t2855: F, t4298: F, t2849: F, t8459: F) -> (F, F, F, F, F, F) {
    let t12598 = t1150 * t12597;
    let t12600 = t1119 * t141;
    let t12601 = t3233 * t12600;
    let t12606 = t4456 * t11325;
    let t12612 = t4463 * t11325;
    let t12617 = t4298 * t2855;
    let t12621 = t8459 * t2849;
    (t12598, t12601, t12606, t12612, t12617, t12621)
}
