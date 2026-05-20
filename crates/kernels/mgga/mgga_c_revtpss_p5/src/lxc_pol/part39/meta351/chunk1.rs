//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1203/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1203<F: Float>(t14363: F, t775: F, t890: F, t1469: F, t749: F, t606: F, t4401: F, t10561: F, t10563: F, t2394: F, t262: F, t10569: F) -> (F, F, F, F, F, F, F) {
    let t14364 = F::cast_from(0.10843581300301739842e-1_f64) * t14363;
    let t14365 = t890 * t775;
    let t14369 = t749 * t1469;
    let t14370 = t14369 * t606;
    let t14372 = F::new(24.0) * t4401 * t14370;
    let t14373 = F::new(8.0) * t10561;
    let t14374 = F::new(2.0) * t10563;
    let t14375 = t2394 * t262;
    let t14379 = F::cast_from(0.4883052614935078681e-3_f64) * t10569;
    (t14364, t14365, t14372, t14373, t14374, t14375, t14379)
}
