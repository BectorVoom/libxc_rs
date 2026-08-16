//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1472/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472<F: Float>(t17361: F, t5293: F, t1261: F, t12879: F, t247: F, t6425: F, t17416: F, t5391: F, t44693: F, t6421: F, t1222: F, t6652: F, t697: F) -> (F, F, F, F, F) {
    let t69971 = t5293 * t17361;
    let t70032 = t1261 * t247 * t12879 * t6425;
    let t70112 = t5391 * t17416;
    let t70133 = t1261 * t247 * t44693 * t6421;
    let t70225 = t1222 * t697 * t6652;
    (t69971, t70032, t70112, t70133, t70225)
}
