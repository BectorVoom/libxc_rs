//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1034/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1034<F: Float>(t1775: F, t18359: F, t15921: F, t5497: F, t4644: F, t7627: F, t5005: F, t964: F, t15909: F, t5486: F, t2063: F, t5515: F, t5491: F, t2023: F, t220: F, t7246: F) -> (F, F, F, F, F, F, F, F) {
    let t18360 = t1775 * t18359;
    let t18363 = t5497 * t15921;
    let t18364 = t1775 * t18363;
    let t18367 = t7627 * t4644;
    let t18368 = t1775 * t18367;
    let t18372 = t964 * t5005;
    let t18373 = t5486 * t15909;
    let t18374 = t18372 * t18373;
    let t18377 = t2063 * t5515;
    let t18378 = t5491 * t18377;
    let t18379 = t1775 * t18378;
    let t18382 = t220 * t2023;
    let t18383 = t5491 * t18382;
    let t18384 = t7246 * t18383;
    (t18360, t18364, t18368, t18374, t18377, t18379, t18382, t18384)
}
