//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1258/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1258<F: Float>(t2266: F, t7040: F, t7145: F, t19444: F, t970: F, t1543: F, t2858: F, t6955: F, t18856: F, t18786: F, t18839: F, t18843: F, t18855: F, t19439: F, t23320: F, t23321: F, t23685: F) -> (F, F, F, F) {
    let t23688 = 18.0 * t2266 * t7040 * t7145;
    let t23689 = t19444 * t970;
    let t23693 = 18.0 * t2858 * t6955 * t1543;
    let t23694 = 12.0 * t18856;
    let t23695 = t18786 - t23320 - t23321 + t18839 - t18843 + 3.0 * t19439 + t23685 + t23688 - 0.2363e1 * t23689 + t18855 - t23693 - t23694;
    (t23688, t23693, t23694, t23695)
}
