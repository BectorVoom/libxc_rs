//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 989/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk989<F: Float>(t1707: F, t17468: F, t1248: F, t16017: F, t1720: F, t16013: F, t4893: F, t11003: F, t15999: F, t1774: F, t3117: F, t16009: F, t16026: F, t16022: F, t7130: F, t1714: F) -> (F, F, F, F, F, F, F, F) {
    let t17469 = t1707 * t17468;
    let t17472 = t1248 * t1720 * t16017;
    let t17475 = t1248 * t4893 * t16013;
    let t17478 = t1248 * t11003 * t15999;
    let t17480 = t3117 * t1774;
    let t17482 = t1248 * t17480 * t16009;
    let t17485 = t1248 * t1720 * t16026;
    let t17488 = t1248 * t7130 * t16022;
    let t17492 = t1714 * t17468;
    (t17469, t17472, t17475, t17478, t17482, t17485, t17488, t17492)
}
