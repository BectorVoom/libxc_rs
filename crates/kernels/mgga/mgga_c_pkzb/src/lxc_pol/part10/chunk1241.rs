//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1241/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1241<F: Float>(t17043: F, t9000: F, t16399: F, t8916: F, t164: F, t8888: F, t600: F, t1753: F, t3441: F, t5257: F, t8906: F, t6966: F, t8911: F, t1719: F, t3396: F, t3410: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24282 = t17043 * t9000;
    let t24298 = t16399 * t8916;
    let t24300 = t8888 * t164;
    let t24311 = t8888 * t600 * t164;
    let t24316 = t3441 * t1753 * t164;
    let t24320 = t5257 * t8906;
    let t24322 = t6966 * t8911;
    let t24324 = t3396 * t1719;
    let t24337 = t3410 * t1719 * t164;
    (t24282, t24298, t24300, t24311, t24316, t24320, t24322, t24324, t24337)
}
