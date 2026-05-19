//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 559/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk559<F: Float>(t2666: F, t546: F, t565: F, t2177: F, t924: F, t1592: F, t2152: F, t2159: F, t2166: F, t2178: F, t2192: F, t2204: F, t2210: F, t2212: F, t2216: F, t2646: F, t2651: F, t2656: F, t2662: F, t562: F, t566: F, t568: F, t574: F, t576: F) -> (F, F, F) {
    let t2667 = t546 * t2666;
    let t2670 = t565 * t2666;
    let t2675 = t2177 * t924;
    let t2681 = -F::cast_from(0.43341108700271342816e-1_f64) * t574 * t2646 - F::cast_from(0.43341108700271342816e-1_f64) * t2651 * t576 + F::cast_from(0.13002332610081402845e0_f64) * t1592 * t2656 - F::cast_from(0.58218257753910989057e-2_f64) * t2152 - F::cast_from(0.48787202696913915093e-2_f64) * t2159 - t2166 - F::cast_from(0.13002332610081402845e0_f64) * t566 * t2662 - F::cast_from(0.43341108700271342816e-1_f64) * t2667 * t562 - F::cast_from(0.13002332610081402845e0_f64) * t2670 * t568 + F::cast_from(0.12805040077930161442e0_f64) * t2178 + F::cast_from(0.11557628986739024751e0_f64) * t2192 + F::cast_from(0.12805040077930161442e0_f64) * t2675 - F::cast_from(0.58218257753910989057e-2_f64) * t2204 - F::cast_from(0.17465477326173296717e-1_f64) * t2210 + F::cast_from(0.27439371595564631661e-2_f64) * t2212 - F::cast_from(0.97574405393827830186e-2_f64) * t2216;
    (t2667, t2670, t2681)
}
