//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 725/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk725<F: Float>(t12051: F, t357: F, t11239: F, t3143: F, t342: F, t3154: F, t4980: F, t994: F, t4995: F, t3057: F, t3286: F, t11627: F, t11631: F, t1129: F, t3431: F, t408: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12052 = t12051 * t357;
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12122 = t994 * t4980;
    let t12127 = t994 * t4995;
    let t12149 = t3057 * t3286;
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12168 = t12051 * t11631;
    let t12226 = 1.0 / t3431 / t1129;
    let t12227 = t408 * t12226;
    (t12052, t12078, t12079, t12122, t12127, t12149, t12167, t12168, t12227)
}
