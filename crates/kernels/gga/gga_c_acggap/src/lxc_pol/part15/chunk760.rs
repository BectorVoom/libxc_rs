//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 760/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk760<F: Float>(t1839: F, t599: F, t1181: F, t2068: F, t1165: F, t604: F, t1815: F, t7413: F, t1849: F, t7351: F, t7575: F, t1713: F, t142: F, t7450: F, t2313: F, t507: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9636 = t599 * t1839;
    let t9637 = t1181 * t9636;
    let t9638 = t2068 * t9637;
    let t9641 = t1165 * t604 * t1839;
    let t9642 = t2068 * t9641;
    let t9645 = t1165 * t604 * t1815;
    let t9646 = t7413 * t9645;
    let t9648 = t599 * t1815;
    let t9649 = t1181 * t9648;
    let t9650 = t7413 * t9649;
    let t9653 = t1165 * t7351 * t1849;
    let t9654 = t7575 * t9653;
    let t9659 = t599 * t1713;
    let t9660 = t142 * t9659;
    let t9661 = t7450 * t9660;
    let t9663 = t507 * t2313;
    (t9636, t9637, t9638, t9641, t9642, t9645, t9646, t9648, t9649, t9650, t9653, t9654, t9659, t9660, t9661, t9663)
}
