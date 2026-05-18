//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 837/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk837<F: Float>(t2024: F, t2029: F, t6479: F, t2004: F, t680: F, t2011: F, t677: F, t2187: F, t783: F, t222: F, t226: F, t6007: F) -> (F, F, F, F, F) {
    let t6481 = t2024 * t6479 * t2029;
    let t6483 = t2004 * t680;
    let t6485 = t677 * t2011;
    let t6497 = t783 * t2187;
    let t6527 = t222 * t6007 * t226;
    (t6481, t6483, t6485, t6497, t6527)
}
