//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2394/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2394<F: Float>(t1828: F, t3736: F, t3575: F, t3790: F, t3737: F, t1811: F, t3566: F) -> (F, F, F, F, F) {
    let t17987 = t3736 * t1828;
    let t17988 = t17987 * t3575;
    let t17991 = t1828 * t3790;
    let t17992 = t3737 * t17991;
    let t17995 = t3566 * t1811;
    (t17987, t17988, t17991, t17992, t17995)
}
