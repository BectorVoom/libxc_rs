//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 833/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk833<F: Float>(t164: F, t51: F, t592: F, t8888: F, t1727: F, t3448: F, t1020: F, t179: F, t6970: F, t2575: F, t2600: F, t3441: F) -> (F, F, F, F, F) {
    let t8891 = t592 * t51 * t8888 * t164;
    let t8894 = t1727 * t3448;
    let t8897 = t179 * t6970 * t1020;
    let t8901 = t179 * t2600 * t2575;
    let t8904 = t3441 * t164;
    (t8891, t8894, t8897, t8901, t8904)
}
