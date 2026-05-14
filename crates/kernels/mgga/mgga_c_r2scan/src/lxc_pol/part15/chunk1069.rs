//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1069/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1069<F: Float>(t1234: F, t3582: F, t3262: F, t3276: F, t37452: F, t37455: F, t40360: F, t40363: F, t40365: F, t40368: F, t40370: F, t40373: F, t40377: F, t40381: F, t40386: F, t40388: F, t40391: F, t40396: F) -> (F, F) {
    let t40397 = t3582 * t1234;
    let t40400 = 15.0 / 16.0 * t3262 * t3276 * t40397;
    let t40401 = -t40360 + t40363 + t40365 - t40368 - t37452 - t40370 + t40373 - t40377 - t40381 - 0.38422568777328955684e-2 * t37455 + t40386 - 0.36021158228745895953e-3 * t40388 - 0.72042316457491791906e-3 * t40391 - t40396 + t40400;
    (t40400, t40401)
}
