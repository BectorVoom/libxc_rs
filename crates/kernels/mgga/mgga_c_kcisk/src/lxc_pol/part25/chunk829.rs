//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 829/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk829<F: Float>(t12261: F, t2024: F, t782: F, t4419: F, t5516: F, t5510: F, t5507: F, t695: F, t1990: F, t5444: F, t2041: F, t5525: F, t2038: F, t5531: F, t2040: F, t798: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12262 = t12261 * t2024;
    let t12263 = t782 * t12262;
    let t12265 = t4419 * t5516;
    let t12266 = t782 * t12265;
    let t12268 = t4419 * t5510;
    let t12269 = t782 * t12268;
    let t12284 = t5507 * t695;
    let t12325 = t1990 * t5444;
    let t12342 = t5525 * t2041;
    let t12345 = t2038 * t5531;
    let t12350 = t2040 * t2040;
    let t12351 = 1.0 / t12350;
    let t12352 = t798 * t12351;
    (t12263, t12266, t12269, t12284, t12325, t12342, t12345, t12350, t12351, t12352)
}
