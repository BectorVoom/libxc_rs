//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1007/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1007<F: Float>(t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1126: F, t3432: F, t418: F, t240: F, t3698: F, t3361: F, t635: F, t1146: F, t2439: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12226 = 1.0 / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12230 = 1.0 / t3434 / t421;
    let t12243 = t1126 * t3432;
    let t12247 = 1.0 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12254 = t240 * t3698;
    let t12256 = 1.0 / t3361 / t635;
    let t12261 = t2439 * t1146;
    (t12166, t12167, t12227, t12230, t12243, t12248, t12254, t12256, t12261)
}
