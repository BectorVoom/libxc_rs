//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1199/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1199(t10621: f64, t164: f64, t1024: f64, t10557: f64, t10561: f64, t16343: f64, t16373: f64, t1706: f64, t1733: f64, t179: f64, t20155: f64, t24272: f64, t24282: f64, t24298: f64, t24320: f64, t24322: f64, t2575: f64, t2586: f64, t2593: f64, t29279: f64, t3396: f64, t3402: f64, t3441: f64, t5225: f64, t5244: f64, t5279: f64, t568: f64, t581: f64, t6758: f64, t8817: f64, t8914: f64, t8962: f64) -> f64 {
    let t29323 = t10621 * t164;
    let t29340 = 0.60023625365297631762e-2_f64 * t24272 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t8962 * t6758 + 0.25724410870841842184e-1_f64 * t16343 * t179 * t8914 * t6758 - 0.10289764348336736873e-1_f64 * t5244 * t179 * t2593 * t29279 + 0.12004725073059526352e0_f64 * t24282 - 0.51448821741683684368e-2_f64 * t5244 * t179 * t2593 * t3441 * t568 + t1706 * t581 * t10557 * t568 / 16.0_f64 + 5.0_f64 / 4.0_f64 * t16373 * t581 * t10561 * t568 - 3.0_f64 / 4.0_f64 * t5225 * t581 * t3402 * t2575 + 0.85748036236139473944e-3_f64 * t1733 * t179 * t29323 * t568 + 0.24009450146119052705e-1_f64 * t24298 - 0.12004725073059526352e-1_f64 * t24320 + 0.30011812682648815881e-2_f64 * t24322 + 0.11337795902333997111e0_f64 * t20155 + 3.0_f64 / 16.0_f64 * t1706 * t581 * t2586 * t3396 + 3.0_f64 / 16.0_f64 * t1706 * t581 * t1024 * t8817;
    t29340
}
