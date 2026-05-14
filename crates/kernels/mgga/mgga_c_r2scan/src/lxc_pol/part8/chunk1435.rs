//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1435/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1435<F: Float>(t10389: F, t498: F, t5086: F, t9964: F, t11002: F, t1556: F, t1562: F, t2531: F, t2533: F, t2534: F, t2847: F, t285: F, t3053: F, t3056: F, t3060: F, t31237: F, t31281: F, t31317: F, t3229: F, t3270: F, t33063: F, t494: F, t495: F, t5087: F, t7218: F, t792: F, t8691: F, t8701: F, t8707: F, t8723: F, t920: F, t921: F, t9947: F, t9956: F, t9967: F) -> (F,) {
    let t34792 = t498 * t10389;
    let t34795 = t5086 * t9964;
    let t34815 = -15.0 / 16.0 * t921 * t31237 + 3.0 / 4.0 * t2533 * t3270 * t3229 - 5.0 / 16.0 * t1562 * t10389 * t792 + 135.0 / 64.0 * t921 * t31281 + t9947 * t494 * t2534 + 3.0 / 4.0 * t2531 * t8723 - 15.0 / 16.0 * t1562 * t2847 * t3229 - 15.0 / 16.0 * t2533 * t11002 * t3060 + t495 * t34792 / 4.0 + 45.0 / 64.0 * t495 * t34795 - 15.0 / 16.0 * t3053 * t7218 + 3.0 / 4.0 * t8701 * t8707 - 15.0 / 16.0 * t3056 * t7218 + t33063 * t285 + 135.0 / 64.0 * t5087 * t9967 * t792 + 3.0 * t8691 * t920 * t2534 + t9956 * t1556 / 4.0 - 15.0 / 8.0 * t921 * t31317;
    (t34815,)
}
