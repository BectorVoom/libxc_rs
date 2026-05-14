//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1124/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1124<F: Float>(t10261: F, t824: F, t758: F, t3026: F, t3236: F, t179: F, t3757: F, t6398: F, t404: F, t2405: F, t3730: F, t1238: F, t3229: F, t10242: F, t10245: F, t10253: F, t10258: F, t2380: F, t3225: F, t3235: F, t3238: F, t6468: F, t8319: F, t8386: F, t8389: F, t8394: F, t8398: F, t8408: F, t8458: F, t8469: F, t8472: F) -> (F, F, F) {
    let t10262 = t10261 * t824;
    let t10263 = t758 * t10262;
    let t10266 = t3236 * t3026;
    let t10267 = t758 * t10266;
    let t10271 = t179 * t6398 * t3757;
    let t10272 = t404 * t10271;
    let t10275 = t179 * t2405 * t3730;
    let t10276 = t404 * t10275;
    let t10278 = t1238 * t3229;
    let t10280 = -t8386 - 0.47637797908966374413e-4 * t6468 + t8389 - t8394 + 0.28582678745379824648e-3 * t10242 - 0.42874018118069736972e-3 * t2380 * t10245 + 0.45732285992607719436e-2 * t8319 * t3225 + 0.19055119163586549765e-3 * t8398 + 0.12862205435420921092e-2 * t3235 * t10253 - 0.13719685797782315831e-1 * t10258 * t3238 - 0.51448821741683684368e-2 * t3235 * t10263 + 0.25724410870841842184e-2 * t3235 * t10267 + 0.85748036236139473947e-3 * t10272 - 0.28582678745379824648e-3 * t10276 + 0.30488190661738479624e-2 * t10278 + t8408 - t8458 + t8469 - t8472;
    (t10262, t10266, t10280)
}
