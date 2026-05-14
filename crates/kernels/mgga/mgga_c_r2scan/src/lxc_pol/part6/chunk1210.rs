//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1210/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1210<F: Float>(t21115: F, t234: F, t5291: F, t720: F, t1696: F, t1838: F, t5296: F, t732: F, t5260: F, t5299: F, t21380: F, t597: F, t1860: F, t1842: F, t5276: F, t5280: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22312 = 0.4155806185363551302e4 * t234 * t5291 * t720 * t21115;
    let t22313 = t1696 * t1838;
    let t22315 = t732 * t5296;
    let t22319 = 0.24934837112181307812e4 * t234 * t5260 * t5299;
    let t22320 = t597 * t21380;
    let t22321 = t1860 * t22320;
    let t22323 = t1696 * t1842;
    let t22325 = t732 * t5276;
    let t22329 = t732 * t5280;
    (t22312, t22313, t22315, t22319, t22320, t22321, t22323, t22325, t22329)
}
