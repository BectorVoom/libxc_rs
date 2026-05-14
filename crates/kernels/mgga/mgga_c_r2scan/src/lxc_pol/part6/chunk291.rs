//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 291/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk291<F: Float>(t246: F, t381: F, t404: F, t408: F, t412: F, t426: F, t459: F, t461: F, t466: F, t470: F, t607: F, t764: F, t765: F, t766: F, t11: F, t5: F, t581: F, t753: F) -> (F,) {
    let t771 = t764 + 0.675260332e-1 * t765 * t766 - 0.285764e-1 * t246 * t607 - t381 - t404 - t408 + t412 - t426 - t459 - t461 + t466 + t470;
    let t774 = 5.0 * t5 * t11 * t753 - 45.0 * param_eta * t771 - t581;
    (t774,)
}
