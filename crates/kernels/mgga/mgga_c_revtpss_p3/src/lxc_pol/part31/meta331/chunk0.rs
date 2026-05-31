//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1335/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1335<F: Float>(t3057: F, t3286: F, t1071: F, t1086: F, t994: F, t3316: F, t989: F, t11239: F, t11627: F, t342: F, t1129: F, t3431: F) -> (F, F, F, F, F, F) {
    let t12149 = t3057 * t3286;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12160 = t989 * t3316;
    let t12166 = t11239 * t11627;
    let t12167 = t342 * t12166;
    let t12226 = F::cast_from(1.0_f64) / t3431 / t1129;
    (t12149, t12154, t12160, t12166, t12167, t12226)
}
