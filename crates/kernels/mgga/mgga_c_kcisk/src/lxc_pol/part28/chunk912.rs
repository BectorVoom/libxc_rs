//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 912/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk912<F: Float>(t1849: F, t1887: F, t1882: F, t4597: F, t11200: F, t2469: F, t1692: F, t6884: F) -> (F, F, F, F, F) {
    let t16960 = t1887 * t1849;
    let t16969 = t1882 * t1849;
    let t16974 = t1882 * t4597;
    let t17004 = t2469 * t11200;
    let t17010 = t6884 * t1692;
    (t16960, t16969, t16974, t17004, t17010)
}
