//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1031/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1031<F: Float>(t216: F, t5331: F, t5335: F, t5350: F, t5354: F, t5360: F, t5366: F, t5384: F, t5392: F, t5394: F, t5401: F, t5405: F, t7708: F, t7710: F, t7721: F, t7725: F, t7727: F, t7737: F, t9904: F) -> (F,) {
    let t10227 = 24.0 * t7708 + 24.0 * t7710 - t5331 + t5335 - t5350 - t5354 - t5360 - 0.21973736767207854065e-2 * t9904 * t216 + 0.30762056574649219973e4 * t7721 - 0.70178683471615754484e1 * t7725 - 0.3903689268108626343e0 * t7727 + t5366 + 0.19518446340543131715e0 * t7737 + t5384 - t5392 + t5394 - t5401 - t5405;
    (t10227,)
}
