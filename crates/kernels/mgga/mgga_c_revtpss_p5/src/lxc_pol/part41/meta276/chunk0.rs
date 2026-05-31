//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1027/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1027<F: Float>(t10199: F, t112: F, t2289: F, t666: F, t654: F, t98: F, t99: F, t106: F, t107: F, t10: F, t580: F, t22: F, t576: F) -> (F, F, F, F, F, F, F, F) {
    let t10201 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10207 = t654 * t654;
    let t10208 = F::cast_from(1.0_f64) / t10207;
    let t10226 = t99 * t98;
    let t10227 = F::cast_from(1.0_f64) / t10226;
    let t10240 = t107 * t106;
    let t10241 = F::cast_from(1.0_f64) / t10240;
    let t10270 = t10 * t580;
    let t10272 = t576 * t22;
    (t10201, t10202, t10207, t10208, t10227, t10241, t10270, t10272)
}
