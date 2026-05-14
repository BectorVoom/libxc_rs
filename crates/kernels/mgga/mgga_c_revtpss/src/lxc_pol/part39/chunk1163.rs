//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1163/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1163<F: Float>(t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F, t15312: F, t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11422: F, t11423: F, t15221: F, t15230: F) -> (F, F, F, F) {
    let t15315 = -0.34431666666666666667e0 * t15137 - 0.57386111111111111112e0 * t15142 + 0.20659e1 * t15147 + 0.103295e1 * t15151 + 0.20659e1 * t15156 - 0.309885e1 * t15160 + 0.20839e0 * t15163 - 0.62517e0 * t15166 - t15312 + 0.46308888888888888889e-1 * t15170 - 0.69463333333333333334e-1 * t15173;
    let t15322 = 0.34431666666666666666e0 * t15191;
    let t15324 = 0.13892666666666666667e0 * t15197;
    let t15337 = -t11422 - t11423 + 0.6311625e0 * t15221 + 0.23154444444444444444e-1 * t11339 - 0.34431666666666666666e0 * t11138 - 0.45908888888888888888e0 * t11134 + 0.17215833333333333333e0 * t11140 + 0.11477222222222222222e0 * t11136 - 0.23154444444444444444e0 * t11366 + 0.69463333333333333333e-1 * t11368 + 0.3529725e1 * t15230;
    (t15315, t15322, t15324, t15337)
}
