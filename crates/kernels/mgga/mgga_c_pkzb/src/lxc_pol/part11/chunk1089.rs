//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1089/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1089<F: Float>(t10621: F, t164: F, t1024: F, t10557: F, t10561: F, t16343: F, t16373: F, t1706: F, t1733: F, t179: F, t20155: F, t24272: F, t24282: F, t24298: F, t24320: F, t24322: F, t2575: F, t2586: F, t2593: F, t29279: F, t3396: F, t3402: F, t3441: F, t5225: F, t5244: F, t5279: F, t568: F, t581: F, t6758: F, t8817: F, t8914: F, t8962: F) -> (F,) {
    let t29323 = t10621 * t164;
    let t29340 = 0.60023625365297631762e-2 * t24272 - 0.12862205435420921092e-1 * t5279 * t179 * t8962 * t6758 + 0.25724410870841842184e-1 * t16343 * t179 * t8914 * t6758 - 0.10289764348336736873e-1 * t5244 * t179 * t2593 * t29279 + 0.12004725073059526352e0 * t24282 - 0.51448821741683684368e-2 * t5244 * t179 * t2593 * t3441 * t568 + t1706 * t581 * t10557 * t568 / 16.0 + 5.0 / 4.0 * t16373 * t581 * t10561 * t568 - 3.0 / 4.0 * t5225 * t581 * t3402 * t2575 + 0.85748036236139473944e-3 * t1733 * t179 * t29323 * t568 + 0.24009450146119052705e-1 * t24298 - 0.12004725073059526352e-1 * t24320 + 0.30011812682648815881e-2 * t24322 + 0.11337795902333997111e0 * t20155 + 3.0 / 16.0 * t1706 * t581 * t2586 * t3396 + 3.0 / 16.0 * t1706 * t581 * t1024 * t8817;
    (t29340,)
}
