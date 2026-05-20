//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1377/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1377<F: Float>(t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1130: F, t3376: F, t1126: F, t3432: F) -> (F, F, F, F, F, F) {
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12226 = F::new(1.0) / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12230 = F::new(1.0) / t3434 / t421;
    let t12238 = t3376 * t1130;
    let t12243 = t1126 * t3432;
    (t12166, t12167, t12227, t12230, t12238, t12243)
}
