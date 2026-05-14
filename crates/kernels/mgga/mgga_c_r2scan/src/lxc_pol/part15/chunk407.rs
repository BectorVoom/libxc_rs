//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 407/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk407<F: Float>(t732: F, t745: F, t1419: F, t230: F, t1422: F, t1376: F, t1650: F, t1651: F, t1655: F, t1662: F, t1667: F, t1671: F, t1674: F, t1688: F, t1695: F, t1696: F, t216: F, t236: F, t596: F, t598: F) -> (F, F, F) {
    let t1699 = t732 * t745;
    let t1702 = 12.0 * t1419 * t230;
    let t1704 = 32.0 * t1422 * t230;
    let t1705 = t1650 - 0.675260332e-1 * t1651 * t598 - 0.1350520664e0 * t596 * t1655 - t1662 + t1667 - t1671 + 0.16936279733333333333e-2 * t1674 + t1688 - 0.21973736767207854065e-2 * t1376 * t216 - t1695 + 0.5848223622634646207e0 * t1696 * t236 + 0.11696447245269292414e1 * t1699 - t1702 + t1704;
    (t1699, t1702, t1705)
}
