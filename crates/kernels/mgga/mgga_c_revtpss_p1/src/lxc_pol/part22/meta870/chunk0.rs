//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3029/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3029<F: Float>(t14742: F, t2689: F, t243: F, t9794: F, t10760: F, t14495: F, t14587: F, t40799: F, t4372: F, t9789: F, t40627: F, t50451: F) -> (F, F, F, F, F) {
    let t51074 = t2689 * t14742;
    let t51076 = t9794 * t243;
    let t51078 = t10760 * t51076 * t14495;
    let t51081 = t40799 * t51076 * t14587;
    let t51083 = t9789 * t4372;
    let t51086 = t10760 * t40627 * t50451;
    (t51074, t51078, t51081, t51083, t51086)
}
