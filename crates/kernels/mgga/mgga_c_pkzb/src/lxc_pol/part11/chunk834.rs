//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 834/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk834<F: Float>(t179: F, t568: F, t8904: F, t3441: F, t600: F, t164: F, t1721: F, t3410: F) -> (F, F, F, F, F) {
    let t8906 = t179 * t8904 * t568;
    let t8909 = t3441 * t600;
    let t8910 = t8909 * t164;
    let t8911 = t179 * t8910;
    let t8914 = t3410 * t1721;
    (t8906, t8909, t8910, t8911, t8914)
}
