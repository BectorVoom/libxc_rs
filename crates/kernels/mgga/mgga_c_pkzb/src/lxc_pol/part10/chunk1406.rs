//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1406/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1406<F: Float>(t10266: F, t2099: F, t3235: F, t10262: F, t2349: F, t3849: F, t1220: F, t8339: F, t154: F, t2347: F, t385: F, t9795: F, t8329: F, t10261: F, t2185: F, t23388: F, t2354: F, t2358: F, t2371: F, t2380: F, t2381: F, t2387: F, t27287: F, t28256: F, t3185: F, t3236: F, t3730: F, t758: F, t7945: F, t8333: F, t907: F, t921: F) -> (F,) {
    let t28353 = t3235 * t2099 * t10266;
    let t28364 = t3235 * t2099 * t10262;
    let t28374 = t3849 * t2349;
    let t28376 = t1220 * t8339;
    let t28380 = t385 * t154 * t2347 * t9795;
    let t28384 = t1220 * t8329;
    let t28396 = 0.34299214494455789578e-2 * t28353 - 0.51448821741683684368e-2 * t3235 * t758 * t10261 * t2185 + 0.25724410870841842184e-2 * t3235 * t758 * t3236 * t7945 - 0.68598428988911579158e-2 * t28364 - 11.0 / 108.0 * t3849 * t2358 + t1220 * t8333 / 18.0 - t385 * t154 * t907 * t27287 / 96.0 - 11.0 / 162.0 * t28374 - t28376 / 81.0 - t28380 / 144.0 + 11.0 / 54.0 * t3849 * t2354 + t28384 / 27.0 - 0.19055119163586549765e-3 * t23388 - 0.42874018118069736972e-3 * t2380 * t2381 * t3730 * t2387 * t921 - 0.85748036236139473944e-3 * t3185 * t2381 * t28256 * t2371;
    (t28396,)
}
