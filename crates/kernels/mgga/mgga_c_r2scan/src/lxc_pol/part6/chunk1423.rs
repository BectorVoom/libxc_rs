//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1423/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1423<F: Float>(t6006: F, t6007: F, t955: F, t18786: F, t18839: F, t18843: F, t18855: F, t22587: F, t22589: F, t23320: F, t23321: F, t23685: F, t23694: F, t2055: F, t2056: F, t2461: F) -> (F, F) {
    let t26873 = t6006 * t955 * t6007;
    let t26877 = t18786 - t23320 - t23321 + t18839 - t18843 + t23685 + t18855 - t23694 + 0.1714584e0 * t26873 - 0.2025780996e0 * t22587 - 0.2025780996e0 * t22589;
    let t26881 = t2055 * t2461 * t2056;
    (t26877, t26881)
}
