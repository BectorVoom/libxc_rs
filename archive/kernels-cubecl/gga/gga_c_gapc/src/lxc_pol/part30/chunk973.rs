//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 973/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk973<F: Float>(t2763: F, t3137: F, t327: F, t7191: F, t11752: F, t1453: F, t2206: F, t1: F, t311: F, t3383: F, t8676: F, t3756: F, t869: F) -> (F, F, F, F, F, F) {
    let t11755 = t3137 * t327 * t2763 * t7191;
    let t11756 = t11752 * t11755;
    let t11758 = t2206 * t1453;
    let t11759 = t11758 * t1;
    let t11760 = t311 * t11759;
    let t11761 = t8676 * t3383;
    let t11762 = t11760 * t11761;
    let t11764 = t869 * t3756;
    (t11755, t11756, t11759, t11761, t11762, t11764)
}
