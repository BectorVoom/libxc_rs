//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 613/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk613<F: Float>(t2177: F, t924: F, t1592: F, t2152: F, t2159: F, t2166: F, t2178: F, t2192: F, t2204: F, t2210: F, t2212: F, t2216: F, t2646: F, t2651: F, t2656: F, t2662: F, t2667: F, t2670: F, t562: F, t566: F, t568: F, t574: F, t576: F) -> (F, F) {
    let t2675 = t2177 * t924;
    let t2681 = -0.43341108700271342816e-1 * t574 * t2646 - 0.43341108700271342816e-1 * t2651 * t576 + 0.13002332610081402845e0 * t1592 * t2656 - 0.58218257753910989057e-2 * t2152 - 0.48787202696913915093e-2 * t2159 - t2166 - 0.13002332610081402845e0 * t566 * t2662 - 0.43341108700271342816e-1 * t2667 * t562 - 0.13002332610081402845e0 * t2670 * t568 + 0.12805040077930161442e0 * t2178 + 0.11557628986739024751e0 * t2192 + 0.12805040077930161442e0 * t2675 - 0.58218257753910989057e-2 * t2204 - 0.17465477326173296717e-1 * t2210 + 0.27439371595564631661e-2 * t2212 - 0.97574405393827830186e-2 * t2216;
    (t2675, t2681)
}
