//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1248/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1248<F: Float>(t2372: F, t409: F, t2358: F, t6630: F, t298: F, t40: F, t2368: F, t6644: F, t308: F, t1216: F, t1218: F, t1249: F, t1257: F, t1261: F, t23366: F, t23376: F, t23382: F, t23384: F, t23394: F, t35: F, t6621: F, t6635: F, t806: F, t810: F, t8315: F, t8336: F, t8377: F, t8385: F) -> (F, F, F, F, F, F) {
    let t23399 = t2372 * t409;
    let t23400 = 20.0 * t23399;
    let t23407 = t2358 * t6630;
    let t23409 = t298 * t40;
    let t23411 = t2368 * t6644;
    let t23413 = t308 * t40;
    let t23415 = 40.0 / 81.0 * t23366 - 10.0 / 9.0 * t8315 * t806 * t1218 - 10.0 / 9.0 * t6621 * t35 * t1216 * t1249 + 10.0 / 3.0 * t23376 + 10.0 / 3.0 * t8377 * t1216 * t1218 + t23382 + 40.0 / 81.0 * t23384 - 10.0 / 9.0 * t8336 * t810 * t1261 + 10.0 / 9.0 * t6635 * t35 * t1216 * t1257 - 10.0 / 3.0 * t23394 - 10.0 / 3.0 * t8385 * t1216 * t1261 - t23400 - 10.0 * t8377 * t40 * t806 + 10.0 * t8385 * t40 * t810 + 10.0 / 9.0 * t23407 - 10.0 * t23409 + 10.0 / 9.0 * t23411 + 10.0 * t23413;
    (t23399, t23407, t23409, t23411, t23413, t23415)
}
