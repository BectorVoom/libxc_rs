//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1248/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1248<F: Float>(t33113: F, t2684: F, t32948: F, t7585: F, t28366: F, t28378: F, t1029: F, t7383: F, t9796: F, t8638: F, t9972: F, t11044: F, t2197: F) -> (F, F, F, F, F, F, F) {
    let t33114 = F::new(0.59584149919750711116e-1) * t33113;
    let t33117 = F::new(0.87421871174939309262e2) * t2684 * t7585 * t32948;
    let t33126 = F::new(0.95857314884801874192e-1) * t28366;
    let t33127 = F::new(0.63904876589867916128e-1) * t28378;
    let t33129 = t9796 * t1029 * t7383;
    let t33130 = F::new(0.38342925953920749676e0) * t33129;
    let t33132 = F::new(0.21450293971110256002e1) * t8638 * t9972;
    let t33134 = F::new(0.23005755572352449806e2) * t2197 * t11044;
    (t33114, t33117, t33126, t33127, t33130, t33132, t33134)
}
