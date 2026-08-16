//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2669/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2669<F: Float>(t14020: F, t3957: F, t2659: F, t5744: F, t816: F, t13792: F, t48863: F, t13920: F, t2661: F, t3992: F, t543: F, t550: F) -> (F, F, F) {
    let t49134 = t3957 * t14020;
    let t49137 = t816 * t2659 * t5744;
    let t49139 = t49137 * t48863 * t13792;
    let t49144 = t2661 * t3992 * t550 * t13920 * t543;
    (t49134, t49139, t49144)
}
