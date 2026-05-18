//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 152/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk152<F: Float>(t471: F, t481: F, t97: F, t108: F, t381: F, t404: F, t408: F, t412: F, t426: F, t459: F, t461: F, t466: F, t470: F) -> (F, F) {
    let t483 = t97 * t471 * t481;
    let t484 = F::new(3.0) * t483;
    let t486 = (t381 + t404 + t408 - t412 + t426 + t459 + t461 - t466 - t470) * t108;
    (t484, t486)
}
