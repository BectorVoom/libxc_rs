//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1167/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1167<F: Float>(t26987: F, t7652: F, t2148: F, t3727: F, t3566: F, t7635: F, t1214: F, t7638: F, t7637: F, t1209: F, t7627: F, t2150: F, t26884: F, t473: F) -> (F, F, F, F, F, F, F) {
    let t26988 = t7652 * t26987;
    let t26991 = t2148 * t3727;
    let t26994 = t3566 * t7635;
    let t26995 = t7638 * t1214;
    let t26996 = t7637 * t26995;
    let t26999 = t1209 * t7627;
    let t27005 = t2150 * t473 * t26884;
    (t26988, t26991, t26994, t26995, t26996, t26999, t27005)
}
