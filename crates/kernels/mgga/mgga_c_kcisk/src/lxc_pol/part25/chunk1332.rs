//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1332/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1332<F: Float>(t17808: F, t33121: F, t33091: F, t34324: F, t5327: F, t6974: F, t15851: F, t1954: F, t4817: F, t7437: F, t5332: F, t6719: F, t10375: F, t2591: F, t2454: F, t5283: F) -> (F, F, F, F, F, F, F, F) {
    let t117280 = t33121 * t17808;
    let t117282 = t33091 * t34324;
    let t117284 = t6974 * t5327;
    let t117286 = t15851 * t1954;
    let t117288 = t4817 * t7437;
    let t117290 = t6719 * t5332;
    let t117292 = t10375 * t2591;
    let t117294 = t5283 * t2454;
    (t117280, t117282, t117284, t117286, t117288, t117290, t117292, t117294)
}
