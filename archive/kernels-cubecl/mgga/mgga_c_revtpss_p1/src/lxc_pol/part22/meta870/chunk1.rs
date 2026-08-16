//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3030/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030<F: Float>(t10760: F, t40627: F, t50613: F, t14861: F, t9794: F, t10890: F, t4458: F, t10815: F, t4426: F, t40424: F, t4430: F, t14720: F, t9775: F) -> (F, F, F, F, F, F) {
    let t51089 = t10760 * t40627 * t50613;
    let t51092 = t10760 * t9794 * t14861;
    let t51095 = t10890 * t4458;
    let t51098 = t10815 * t4426;
    let t51100 = t40424 * t4430;
    let t51102 = t9775 * t14720;
    (t51089, t51092, t51095, t51098, t51100, t51102)
}
