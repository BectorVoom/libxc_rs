//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1281/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1281<F: Float>(t2666: F, t6527: F, t560: F, t8001: F, t2148: F, t7628: F, t481: F, t6165: F, t2182: F, t775: F, t7630: F, t2837: F, t5095: F, t5096: F, t19793: F, t24006: F, t24016: F, t24018: F, t24022: F, t24025: F, t2598: F, t2662: F, t360: F, t562: F, t568: F, t6127: F, t6490: F, t6530: F, t7433: F, t8031: F) -> (F,) {
    let t24028 = t6527 * t2666;
    let t24031 = t8001 * t560;
    let t24033 = t7628 * t2148 * t24031;
    let t24035 = t8001 * t481;
    let t24037 = t6165 * t2148 * t24035;
    let t24039 = t2182 * t775;
    let t24040 = t24039 * t7630;
    let t24043 = t5095 * t2837 * t5096;
    let t24046 = -0.78013995660488417068e0 * t24006 * t8031 + 0.26004665220162805689e0 * t2598 * t360 * t7433 * t6127 - 0.39006997830244208535e0 * t6490 * t2662 + 0.24393601348456957546e-3 * t24016 + 0.14457274399185490173e-4 * t24018 - 0.13002332610081402845e0 * t24022 * t562 - 0.39006997830244208535e0 * t24025 * t568 - 0.2600466522016280569e1 * t24028 * t6530 + 0.69861909304693186866e-1 * t24033 + 0.1047928639570397803e0 * t24037 + 0.69861909304693186866e-1 * t24040 + 0.20958572791407956061e0 * t24043 - 0.37377291665467981483e0 * t19793;
    (t24046,)
}
