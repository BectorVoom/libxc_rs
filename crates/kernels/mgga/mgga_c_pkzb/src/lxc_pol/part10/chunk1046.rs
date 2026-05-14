//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1046/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1046<F: Float>(t164: F, t51: F, t592: F, t8888: F, t1727: F, t3448: F, t1020: F, t179: F, t6970: F, t2575: F, t2600: F, t3441: F, t568: F, t600: F, t1721: F, t3410: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8891 = t592 * t51 * t8888 * t164;
    let t8894 = t1727 * t3448;
    let t8897 = t179 * t6970 * t1020;
    let t8901 = t179 * t2600 * t2575;
    let t8904 = t3441 * t164;
    let t8906 = t179 * t8904 * t568;
    let t8909 = t3441 * t600;
    let t8910 = t8909 * t164;
    let t8911 = t179 * t8910;
    let t8914 = t3410 * t1721;
    (t8891, t8894, t8897, t8901, t8904, t8906, t8909, t8910, t8911, t8914)
}
