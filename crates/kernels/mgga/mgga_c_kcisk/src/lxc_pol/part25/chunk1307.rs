//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1307/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1307<F: Float>(t2464: F, t32935: F, t5043: F, t7261: F, t112176: F, t1799: F, t6986: F, t32903: F, t6662: F, t32948: F, t34045: F, t32921: F, t116223: F, t1869: F, t33027: F, t15893: F, t34159: F) -> (F, F, F, F, F, F, F) {
    let t116756 = t7261 * t32935 * t2464 * t5043;
    let t116762 = t1799 * t112176 * t6986;
    let t116765 = t1799 * t32903 * t6662;
    let t116768 = 0.26805555555555555556e-2 * t32948 * t34045;
    let t116771 = 0.26805555555555555556e-2 * t32921 * t34045;
    let t116773 = t1869 * t116223 * t33027;
    let t116779 = t1869 * t34159 * t15893;
    (t116756, t116762, t116765, t116768, t116771, t116773, t116779)
}
