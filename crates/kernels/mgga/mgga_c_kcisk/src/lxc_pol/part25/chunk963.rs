//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 963/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk963<F: Float>(t5069: F, t6974: F, t1869: F, t4811: F, t6970: F, t2399: F, t4822: F, t5055: F, t6719: F, t5054: F, t10409: F, t6982: F, t4803: F, t6965: F, t1800: F, t10375: F, t2537: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17073 = t6974 * t5069;
    let t17074 = t1869 * t17073;
    let t17076 = t4811 * t6970;
    let t17077 = 0.22109259259259259258e-2 * t17076;
    let t17078 = t2399 * t4822;
    let t17083 = t6719 * t5055;
    let t17084 = t5054 * t17083;
    let t17086 = t10409 * t6982;
    let t17087 = 0.14739506172839506172e-2 * t17086;
    let t17091 = t6965 * t4803;
    let t17092 = t1800 * t17091;
    let t17093 = t1869 * t17092;
    let t17095 = t10375 * t2537;
    (t17074, t17076, t17077, t17078, t17084, t17086, t17087, t17091, t17093, t17095)
}
