//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1197/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1197<F: Float>(t7839: F, t9674: F, t7564: F, t8480: F, t8613: F, t1181: F, t604: F, t6079: F, t7426: F, t6218: F, t7575: F, t6198: F, t7351: F) -> (F, F, F, F, F) {
    let t40490 = t7839 * t9674;
    let t40493 = t7564 * t8480 * t8613;
    let t40497 = t7426 * t1181 * t604 * t6079;
    let t40501 = t7575 * t1181 * t604 * t6218;
    let t40505 = t7564 * t1181 * t7351 * t6198;
    (t40490, t40493, t40497, t40501, t40505)
}
