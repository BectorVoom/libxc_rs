//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 276/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk276<F: Float>(t372: F, t381: F, t404: F, t408: F, t412: F, t426: F, t459: F, t461: F, t466: F, t470: F, t484: F, t607: F, t766: F, t799: F, t880: F, t881: F) -> (F,) {
    let t885 = t880 - 0.2363e1 * t881 * t766 + t372 * t607 - t381 - t404 - t408 + t412 - t426 - t459 - t461 + t466 + t470 - t484 - t799;
    (t885,)
}
