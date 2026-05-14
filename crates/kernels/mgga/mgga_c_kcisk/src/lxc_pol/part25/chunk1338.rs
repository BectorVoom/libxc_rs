//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1338/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1338<F: Float>(t117362: F, t17847: F, t17878: F, t9708: F, t112095: F, t7333: F, t5283: F, t735: F, t15893: F, t16980: F, t7316: F, t9704: F, t33106: F, t34345: F, t17917: F, t1800: F) -> (F, F, F, F, F, F, F) {
    let t117363 = t117362 * t17847;
    let t117365 = t9708 * t17878;
    let t117367 = t112095 * t7333;
    let t117369 = t5283 * t735;
    let t117370 = t117369 * t15893;
    let t117373 = t9704 * t7316 * t16980;
    let t117375 = t34345 * t33106;
    let t117377 = t1800 * t17917;
    (t117363, t117365, t117367, t117370, t117373, t117375, t117377)
}
