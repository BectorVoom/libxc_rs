//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 918/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk918<F: Float>(t164: F, t2639: F, t179: F, t568: F, t1692: F, t2600: F, t600: F, t615: F, t616: F, t6853: F, t1041: F, t5296: F) -> (F, F, F, F, F, F, F, F) {
    let t6970 = t2639 * t164;
    let t6972 = t179 * t6970 * t568;
    let t6976 = t179 * t2600 * t1692;
    let t6979 = t2639 * t600;
    let t6980 = t6979 * t164;
    let t6981 = t179 * t6980;
    let t6985 = t615 * t616 * t6853;
    let t6988 = t5296 * t1041;
    (t6970, t6972, t6976, t6979, t6980, t6981, t6985, t6988)
}
