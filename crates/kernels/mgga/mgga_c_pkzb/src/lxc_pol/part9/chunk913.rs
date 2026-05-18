//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 913/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk913<F: Float>(t1719: F, t2593: F, t179: F, t1721: F, t6875: F, t2583: F, t5221: F, t2586: F, t568: F, t581: F, t1024: F, t1692: F) -> (F, F, F, F, F, F) {
    let t6903 = t2593 * t1719;
    let t6904 = t179 * t6903;
    let t6908 = t179 * t6875 * t1721;
    let t6914 = F::new(7.0) / F::new(24.0) * t5221 * t2583;
    let t6916 = t581 * t2586 * t568;
    let t6920 = t581 * t1024 * t1692;
    (t6903, t6904, t6908, t6914, t6916, t6920)
}
