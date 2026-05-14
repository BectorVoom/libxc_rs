//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1323/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1323<F: Float>(t11226: F, t654: F, t1869: F, t2563: F, t5063: F, t22999: F, t33017: F, t4972: F, t116320: F, t33056: F, t2788: F, t642: F, t15898: F, t5182: F, t17027: F, t6674: F) -> (F, F, F, F, F) {
    let t117090 = t11226 * t654;
    let t117093 = t1869 * t117090 * t2563 * t5063;
    let t117097 = t1869 * t33017 * t22999 * t4972;
    let t117106 = t33056 * t116320;
    let t117108 = t2788 * t642;
    let t117110 = t5182 * t117108 * t15898;
    let t117113 = t6674 * t117108 * t17027;
    (t117093, t117097, t117106, t117110, t117113)
}
