//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 747/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk747<F: Float>(t10199: F, t112: F, t654: F, t98: F, t99: F, t106: F, t107: F, t10: F, t580: F, t22: F, t576: F, t15: F, t588: F) -> (F, F, F, F, F, F, F) {
    let t10201 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t10199 * t112;
    let t10207 = t654 * t654;
    let t10208 = F::cast_from(1.0_f64) / t10207;
    let t10226 = t99 * t98;
    let t10227 = F::cast_from(1.0_f64) / t10226;
    let t10240 = t107 * t106;
    let t10241 = F::cast_from(1.0_f64) / t10240;
    let t10270 = t10 * t580;
    let t10271 = F::cast_from(12.0_f64) * t10270;
    let t10272 = t576 * t22;
    let t10273 = F::cast_from(36.0_f64) * t10272;
    let t10275 = F::cast_from(24.0_f64) * t15 * t588;
    (t10201, t10208, t10227, t10241, t10271, t10273, t10275)
}
