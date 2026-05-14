//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1258/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1258<F: Float>(t3929: F, t5798: F, t4346: F, t6579: F, t1556: F, t21884: F, t1300: F, t397: F, t2326: F, t4376: F, t21962: F, t4350: F, t14608: F, t2306: F, t1610: F, t22047: F) -> (F, F, F, F, F, F, F, F) {
    let t54621 = t5798 * t3929;
    let t55345 = t6579 * t4346;
    let t55401 = t21884 * t1556;
    let t55867 = t1300 * t397;
    let t56066 = t2326 * t4376;
    let t56777 = t21962 * t4350;
    let t56817 = t2306 * t14608;
    let t57158 = t22047 * t1610;
    (t54621, t55345, t55401, t55867, t56066, t56777, t56817, t57158)
}
