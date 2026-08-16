//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2801/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2801<F: Float>(t4119: F, t4255: F, t41008: F, t5550: F, t16783: F, t41196: F, t118: F, t16662: F, t2576: F, t794: F, t16787: F, t2563: F) -> (F, F, F, F, F) {
    let t59198 = t4255 * t4119;
    let t59204 = t41008 * t5550;
    let t59206 = t41196 * t16783;
    let t59214 = t2576 * t118 * t794 * t16662;
    let t59216 = t2563 * t16787;
    (t59198, t59204, t59206, t59214, t59216)
}
