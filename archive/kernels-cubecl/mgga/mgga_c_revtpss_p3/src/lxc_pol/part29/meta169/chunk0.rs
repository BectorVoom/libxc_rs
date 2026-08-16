//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 817/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk817<F: Float>(t1122: F, t3634: F, t247: F, t1261: F, t1264: F, t3372: F, t3368: F, t1230: F, t1260: F) -> (F, F, F, F, F) {
    let t3635 = t3634 * t1122;
    let t3636 = t247 * t3635;
    let t3637 = t1261 * t3636;
    let t3639 = t1264 * t3372;
    let t3640 = t247 * t3639;
    let t3643 = t1264 * t3368;
    let t3644 = t247 * t3643;
    let t3647 = t1230 * t1260;
    (t3636, t3637, t3640, t3644, t3647)
}
