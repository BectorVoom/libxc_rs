//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 820/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk820<F: Float>(t1556: F, t1562: F, t2531: F, t2533: F, t2534: F, t2538: F, t2847: F, t285: F, t3053: F, t3056: F, t3060: F, t3229: F, t495: F, t499: F, t5087: F, t7218: F, t7221: F, t792: F, t8692: F, t8694: F, t8698: F, t8701: F, t8707: F, t8714: F, t8723: F, t921: F, t9560: F, t983: F) -> (F,) {
    let t9563 = t8692 * t285 + t8694 * t2534 + t3053 * t1556 / 4.0 + 2.0 * t921 * t8698 + t8701 * t2534 + t3056 * t1556 / 4.0 + t2531 * t2538 / 2.0 + t2533 * t8707 / 2.0 - 5.0 / 8.0 * t921 * t7218 + t921 * t7221 / 2.0 - 5.0 / 16.0 * t495 * t8714 + 45.0 / 64.0 * t5087 * t3060 * t792 - 5.0 / 8.0 * t1562 * t983 * t2847 + t495 * t8723 / 4.0 - 5.0 / 16.0 * t1562 * t3229 * t792 + t499 * t9560 / 4.0;
    (t9563,)
}
