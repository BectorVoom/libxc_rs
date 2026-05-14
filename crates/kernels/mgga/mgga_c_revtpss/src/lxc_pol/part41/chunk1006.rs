//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1006/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1006<F: Float>(t3361: F, t57: F, t268: F, t404: F, t7021: F, t1123: F, t2435: F) -> (F, F, F, F) {
    let t12267 = t3361 * t57;
    let t12268 = 1.0 / t12267;
    let t12295 = t268 * t7021 * t404;
    let t12296 = 28.0 / 27.0 * t12295;
    let t12297 = t2435 * t1123;
    (t12268, t12295, t12296, t12297)
}
