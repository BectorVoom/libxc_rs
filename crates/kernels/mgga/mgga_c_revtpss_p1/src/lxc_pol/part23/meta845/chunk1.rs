//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2725/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2725<F: Float>(t1235: F, t127: F, t21083: F, t371: F, t12967: F, t20846: F, t17708: F, t59550: F, t12916: F, t21299: F, t3718: F, t20842: F, t3667: F) -> (F, F, F, F, F) {
    let t70521 = t1235 * t371 * t127 * t21083;
    let t70523 = t12967 * t20846;
    let t70530 = t59550 * t17708;
    let t70542 = t3718 * t12916 * t21299;
    let t70581 = t3667 * t20842;
    (t70521, t70523, t70530, t70542, t70581)
}
