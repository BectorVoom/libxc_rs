//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2981/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2981<F: Float>(t14230: F, t2782: F, t46456: F, t1385: F, t14066: F, t14155: F, t1432: F, t2470: F, t1892: F, t4056: F, t4086: F, t543: F) -> (F, F, F, F) {
    let t49263 = t2782 * t46456 * t14230;
    let t49268 = t1385 * t14066;
    let t49273 = t1432 * t14155 * t2470;
    let t49280 = t1892 * t4056;
    let t49283 = t2782 * t4086 * t49280 * t543;
    (t49263, t49268, t49273, t49283)
}
