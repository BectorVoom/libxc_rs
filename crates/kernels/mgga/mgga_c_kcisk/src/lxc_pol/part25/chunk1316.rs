//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1316/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1316<F: Float>(t17759: F, t1799: F, t9679: F, t18325: F, t34072: F, t34153: F, t1333: F, t34242: F, t2509: F, t415: F, t4804: F, t112728: F, t2528: F, t32942: F, t34097: F, t1763: F, t1772: F, t7278: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116958 = t1799 * t9679 * t17759;
    let t116960 = t34072 * t18325;
    let t116965 = t34153 * t18325;
    let t116970 = t1333 * t34242;
    let t116971 = 0.88437037037037037034e-2 * t116970;
    let t116973 = t415 * t2509 * t4804;
    let t116976 = t415 * t112728 * t2528;
    let t116979 = 0.69444444444444444446e-2 * t32942 * t34097;
    let t116983 = t7278 * t1763 * t1772;
    (t116958, t116960, t116965, t116970, t116971, t116973, t116976, t116979, t116983)
}
