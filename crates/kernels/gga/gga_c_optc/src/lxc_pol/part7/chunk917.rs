//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 917/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk917<F: Float>(t116: F, t3241: F, t3242: F, t11899: F, t2849: F, t1119: F, t141: F, t3233: F, t2855: F, t3117: F, t11325: F, t4456: F, t4463: F, t4298: F, t8459: F, t4434: F, t7448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12567 = t3241 * t3242 * t116;
    let t12568 = t11899 * t2849;
    let t12600 = t1119 * t141;
    let t12601 = t3233 * t12600;
    let t12602 = t3117 * t2855;
    let t12606 = t4456 * t11325;
    let t12612 = t4463 * t11325;
    let t12617 = t4298 * t2855;
    let t12621 = t8459 * t2849;
    let t12741 = t4434 * t7448;
    (t12567, t12568, t12601, t12602, t12606, t12612, t12617, t12621, t12741)
}
