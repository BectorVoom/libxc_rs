//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1033/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1033<F: Float>(t229: F, t9904: F, t41: F, t5474: F, t5479: F, t5585: F, t5601: F, t5605: F, t5609: F, t5612: F, t5614: F, t5669: F, t5846: F, t5853: F, t5864: F, t5868: F, t5884: F, t7785: F, t7795: F) -> (F, F) {
    let t10241 = t9904 * t229;
    let t10244 = 0.127022098e-2 * t7785 + t5474 - t41 * t10241 - t5479 - t5846 + t5853 - t5585 - t5864 - t5601 - t5605 + t5609 + t5612 - t5614 - 3.0 * t7795 + t5868 + t5884 - t5669;
    (t10241, t10244)
}
