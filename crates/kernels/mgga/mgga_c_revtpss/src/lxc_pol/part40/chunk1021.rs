//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1021/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1021<F: Float>(t3046: F, t3286: F, t3057: F, t1071: F, t1086: F, t994: F, t3316: F, t989: F, t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F) -> (F, F, F, F, F, F, F, F) {
    let t12146 = t3046 * t3286;
    let t12149 = t3057 * t3286;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12160 = t989 * t3316;
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12226 = 1.0 / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12230 = 1.0 / t3434 / t421;
    (t12146, t12149, t12154, t12160, t12166, t12167, t12227, t12230)
}
