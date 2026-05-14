//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1359/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1359<F: Float>(t2344: F, t3849: F, t10122: F, t8428: F, t926: F, t10076: F, t8435: F, t2370: F, t8309: F, t2368: F, t3757: F, t10044: F, t10075: F, t10340: F, t1167: F, t1227: F, t18649: F, t2226: F, t23122: F, t2371: F, t2380: F, t2381: F, t2382: F, t2387: F, t2396: F, t300: F, t3185: F, t3186: F, t3206: F, t3913: F, t406: F, t6366: F, t6404: F, t8261: F, t8382: F, t8445: F, t8450: F, t919: F, t921: F, t9795: F) -> (F, F) {
    let t27175 = t3849 * t2344;
    let t27178 = t8428 * t926 * t10122;
    let t27181 = t8435 * t926 * t10076;
    let t27187 = t2370 * t8309;
    let t27198 = t3757 * t2368;
    let t27220 = -0.85748036236139473944e-3 * t2380 * t2381 * t9795 * t919 * t921 + 0.21437009059034868486e-3 * t8450 * t406 * t10075 * t8445 - 0.91464571985215438872e-2 * t10044 * t8382 - 11.0 / 486.0 * t27175 + 0.17149607247227894789e-2 * t27178 - 0.17149607247227894789e-2 * t27181 - 0.85748036236139473944e-3 * t3185 * t2381 * t3913 * t18649 + 0.85748036236139473944e-3 * t3185 * t406 * t3186 * t27187 + 0.28582678745379824648e-3 * t23122 + 0.12862205435420921092e-2 * t2380 * t6366 * t3757 * t2387 * t921 + 0.25724410870841842184e-2 * t3185 * t6366 * t27198 * t2371 - 0.12862205435420921092e-2 * t3206 * t6366 * t27198 * t2396 - 0.85748036236139473944e-3 * t2380 * t2381 * t10340 * t2382 - 0.10289764348336736873e-1 * t2380 * t300 * t6404 * t1227 * t921 * t1167 * t2226 + 0.18292914397043087774e-1 * t10044 * t8261;
    (t27187, t27220)
}
