//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2650/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650<F: Float>(t2661: F, t3992: F, t4057: F, t5608: F, t4004: F, t5651: F, t9934: F, t47198: F, t5665: F, t5629: F, t9779: F, t5661: F, t9909: F) -> (F, F, F, F, F) {
    let t48786 = t2661 * t3992 * t5608 * t4057;
    let t48790 = t2661 * t9934 * t5651 * t4004;
    let t48792 = t47198 * t5665;
    let t48794 = t9779 * t5629;
    let t48796 = t9909 * t5661;
    (t48786, t48790, t48792, t48794, t48796)
}
