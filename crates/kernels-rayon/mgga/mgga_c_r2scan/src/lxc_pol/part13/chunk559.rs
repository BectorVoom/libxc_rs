//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 559/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk559(t2666: f64, t546: f64, t565: f64, t2177: f64, t924: f64, t1592: f64, t2152: f64, t2159: f64, t2166: f64, t2178: f64, t2192: f64, t2204: f64, t2210: f64, t2212: f64, t2216: f64, t2646: f64, t2651: f64, t2656: f64, t2662: f64, t562: f64, t566: f64, t568: f64, t574: f64, t576: f64) -> (f64, f64, f64) {
    let t2667 = t546 * t2666;
    let t2670 = t565 * t2666;
    let t2675 = t2177 * t924;
    let t2681 = -0.43341108700271342816e-1_f64 * t574 * t2646 - 0.43341108700271342816e-1_f64 * t2651 * t576 + 0.13002332610081402845e0_f64 * t1592 * t2656 - 0.58218257753910989057e-2_f64 * t2152 - 0.48787202696913915093e-2_f64 * t2159 - t2166 - 0.13002332610081402845e0_f64 * t566 * t2662 - 0.43341108700271342816e-1_f64 * t2667 * t562 - 0.13002332610081402845e0_f64 * t2670 * t568 + 0.12805040077930161442e0_f64 * t2178 + 0.11557628986739024751e0_f64 * t2192 + 0.12805040077930161442e0_f64 * t2675 - 0.58218257753910989057e-2_f64 * t2204 - 0.17465477326173296717e-1_f64 * t2210 + 0.27439371595564631661e-2_f64 * t2212 - 0.97574405393827830186e-2_f64 * t2216;
    (t2667, t2670, t2681)
}
