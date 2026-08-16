//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3157/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3157<F: Float>(t1145: F, t141: F, t56224: F, t16907: F, t698: F, t16886: F, t16889: F, t12254: F, t56179: F, t56161: F, t56157: F, t56165: F) -> (F, F, F, F, F, F, F, F) {
    let t58202 = t141 * t1145 * t56224;
    let t58207 = t698 * t16907;
    let t58209 = t698 * t16886;
    let t58211 = t698 * t16889;
    let t58214 = t141 * t12254 * t56179;
    let t58217 = t141 * t1145 * t56161;
    let t58220 = t141 * t1145 * t56157;
    let t58223 = t141 * t1145 * t56165;
    (t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223)
}
