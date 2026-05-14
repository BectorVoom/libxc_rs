//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1287/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1287<F: Float>(t112586: F, t17036: F, t5182: F, t112176: F, t1869: F, t34085: F, t2571: F, t33017: F, t4797: F, t62249: F, t9921: F, t9664: F, t15922: F, t5054: F, t9679: F, t10461: F, t15931: F) -> (F, F, F, F, F, F, F) {
    let t116336 = t5182 * t112586 * t17036;
    let t116340 = t1869 * t112176 * t34085;
    let t116344 = t1869 * t33017 * t2571 * t4797;
    let t116350 = t62249 * t9921;
    let t116351 = t9664 * t116350;
    let t116354 = t5054 * t9679 * t15922;
    let t116357 = t10461 * t9679 * t15931;
    (t116336, t116340, t116344, t116350, t116351, t116354, t116357)
}
