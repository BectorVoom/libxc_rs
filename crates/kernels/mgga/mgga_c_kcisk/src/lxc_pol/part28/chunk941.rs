//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 941/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk941<F: Float>(t1894: F, t7715: F, t6666: F, t5184: F, t5182: F, t1757: F, t11682: F, t5192: F, t10409: F, t8486: F, t5185: F, t7718: F, t2507: F, t4594: F, t1336: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22315 = t7715 * t1894;
    let t22316 = t6666 * t22315;
    let t22317 = t5184 * t22316;
    let t22318 = t5182 * t22317;
    let t22320 = t7715 * t1757;
    let t22321 = t11682 * t22320;
    let t22322 = t5192 * t22321;
    let t22323 = t5182 * t22322;
    let t22328 = t10409 * t8486;
    let t22331 = t5185 * t7718 * t1894;
    let t22332 = t5184 * t22331;
    let t22333 = t5182 * t22332;
    let t22335 = t4594 * t2507;
    let t22337 = t140 * t1336 * t22335;
    (t22315, t22316, t22318, t22320, t22321, t22323, t22328, t22331, t22333, t22337)
}
