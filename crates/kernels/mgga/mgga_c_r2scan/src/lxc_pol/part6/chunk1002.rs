//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1002/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1002<F: Float>(t216: F, t5248: F, t5253: F, t5256: F, t5258: F, t5263: F, t5274: F, t5278: F, t5282: F, t5283: F, t5288: F, t5295: F, t7007: F, t1831: F, t963: F, t2747: F, t750: F) -> (F, F, F) {
    let t7681 = -0.21973736767207854065e-2 * t7007 * t216 + t5248 - 0.8103123984e0 * t5253 + 0.1350520664e0 * t5256 + 0.20508037716432813316e4 * t5258 + t5263 + t5274 - t5278 + t5282 - 0.11696447245269292414e1 * t5283 - t5288 - t5295;
    let t7685 = t963 * t1831;
    let t7688 = 0.34631718211362927518e2 * t2747 * t750;
    (t7681, t7685, t7688)
}
