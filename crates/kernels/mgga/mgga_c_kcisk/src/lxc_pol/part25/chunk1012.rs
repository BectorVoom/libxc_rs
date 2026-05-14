//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1012/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1012<F: Float>(t16653: F, t7316: F, t7315: F, t17126: F, t5322: F, t7429: F, t16617: F, t7311: F, t7310: F, t2560: F, t5299: F, t2568: F, t5274: F, t5327: F, t7320: F, t2586: F, t5326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17899 = t7316 * t16653;
    let t17900 = t7315 * t17899;
    let t17902 = t5322 * t17126;
    let t17903 = t7429 * t17902;
    let t17905 = t7311 * t16617;
    let t17906 = t7310 * t17905;
    let t17908 = t2560 * t5299;
    let t17910 = t5274 * t2568;
    let t17912 = t7320 * t5327;
    let t17914 = t2586 * t5326;
    (t17899, t17900, t17902, t17903, t17905, t17906, t17908, t17910, t17912, t17914)
}
