//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 780/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk780<F: Float>(t5585: F, t5601: F, t5605: F, t5609: F, t5612: F, t5614: F, t5669: F, t5855: F, t5864: F, t5868: F, t7795: F, t7796: F, t76: F, t8590: F, t3142: F, t745: F) -> (F, F, F) {
    let t8964 = -t5585 - 0.571528e-1 * t5855 - t5864 - t5601 - t5605 + t5609 + t5612 - t5614 - 2.0 * t7795 + t5868 + t7796 - t5669;
    let t8967 = t8590 * t76;
    let t8970 = t3142 * t745;
    (t8964, t8967, t8970)
}
