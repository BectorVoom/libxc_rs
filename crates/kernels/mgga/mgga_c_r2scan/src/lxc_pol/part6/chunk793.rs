//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 793/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk793<F: Float>(t171: F, t5397: F, t5398: F, t21: F, t502: F, t1684: F, t1680: F, t1651: F, t1655: F, t216: F, t236: F, t4959: F, t5344: F, t5346: F, t5350: F, t5354: F, t5355: F, t5360: F, t5366: F, t5367: F, t5373: F, t5378: F, t5384: F, t5385: F, t5392: F, t5394: F, t598: F) -> (F, F, F, F, F) {
    let t5401 = 0.6858336e0 * t5397 * t171 * t5398;
    let t5402 = t21 * t502;
    let t5403 = t1684 * t5402;
    let t5405 = 0.16936279733333333332e-2 * t1680 * t5403;
    let t5406 = 0.10526802520742363173e2 * t5344 - 0.31168546390226634765e3 * t5346 - t5350 - t5354 - 0.35089341735807877242e1 * t5355 - t5360 - 0.21973736767207854065e-2 * t4959 * t216 + t5366 - 0.675260332e-1 * t5367 * t598 - 0.2025780996e0 * t1651 * t1655 + 0.5143752e0 * t5373 + 0.24012257405919999999e-1 * t5378 + t5384 + 0.5848223622634646207e0 * t5385 * t236 - t5392 + t5394 - t5401 - t5405;
    (t5401, t5402, t5403, t5405, t5406)
}
