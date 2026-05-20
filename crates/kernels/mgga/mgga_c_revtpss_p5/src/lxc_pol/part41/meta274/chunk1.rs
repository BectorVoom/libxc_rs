//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1024/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1024<F: Float>(t10115: F, t557: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t1398: F, t215: F, t268: F, t543: F, t4101: F) -> (F, F, F, F) {
    let t10117 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t557;
    let t10126 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t1429;
    let t10129 = F::cast_from(0.46263278077393568556e-2_f64) * t3964 * t4096 * t9285;
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    (t10117, t10126, t10129, t10137)
}
