//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1183/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1183<F: Float>(t1165: F, t5537: F, t7564: F, t8600: F, t30219: F, t9670: F, t7839: F, t9674: F, t8480: F, t8613: F, t1181: F, t604: F, t6079: F, t7426: F) -> (F, F, F, F, F) {
    let t40485 = t7564 * t1165 * t8600 * t5537;
    let t40487 = t30219 * t9670;
    let t40490 = t7839 * t9674;
    let t40493 = t7564 * t8480 * t8613;
    let t40497 = t7426 * t1181 * t604 * t6079;
    (t40485, t40487, t40490, t40493, t40497)
}
