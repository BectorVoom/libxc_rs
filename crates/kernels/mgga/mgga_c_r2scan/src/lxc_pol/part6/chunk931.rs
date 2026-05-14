//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 931/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk931<F: Float>(t322: F, t1271: F, t1276: F, t1277: F, t1289: F, t321: F, t6649: F, t6651: F, t6654: F, t6661: F, t6662: F, t6665: F, t6679: F, t819: F, t826: F, t1295: F, t829: F) -> (F, F, F) {
    let t324 = 0.0 < t322;
    let t6681 = -3.0 * t1271 * t1289 + 6.0 * t1276 * t6665 + 6.0 * t6654 * t1277 + t6649 * t321 - 3.0 * t6651 * t826 - 6.0 * t6661 * t6662 - t819 * t6679;
    let t6682 = piecewise3(t324, 0.0, t6681);
    let t6688 = t1295 * t829;
    (t6681, t6682, t6688)
}
