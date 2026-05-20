//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 542/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk542<F: Float>(t1179: F, t1749: F, t1756: F, t3523: F, t300: F, t3495: F, t1208: F, t1769: F) -> (F, F, F, F, F) {
    let t5158 = t1749 * t1179;
    let t5184 = t1756 * t3523;
    let t5192 = t300 * t1749;
    let t5197 = t3495 * t1756;
    let t5219 = t1769 * t1208;
    (t5158, t5184, t5192, t5197, t5219)
}
