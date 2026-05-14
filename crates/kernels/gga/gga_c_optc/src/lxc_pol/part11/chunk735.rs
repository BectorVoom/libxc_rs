//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 735/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk735<F: Float>(t1119: F, t141: F, t3233: F, t11325: F, t4456: F, t4463: F, t2855: F, t4298: F, t2849: F, t8459: F, t1497: F, t911: F, t115: F, t3241: F) -> (F, F, F, F, F, F) {
    let t12600 = t1119 * t141;
    let t12601 = t3233 * t12600;
    let t12606 = t4456 * t11325;
    let t12612 = t4463 * t11325;
    let t12617 = t4298 * t2855;
    let t12621 = t8459 * t2849;
    let t12633 = t1497 * t911;
    let t12634 = t12633 * t115;
    let t12635 = t3241 * t12634;
    (t12601, t12606, t12612, t12617, t12621, t12635)
}
