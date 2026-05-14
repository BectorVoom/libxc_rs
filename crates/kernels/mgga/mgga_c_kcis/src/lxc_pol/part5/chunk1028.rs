//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1028/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1028<F: Float>(t278: F, t19164: F, t829: F, t14408: F, t14395: F, t330: F, t1003: F, t14401: F, t19107: F, t1001: F, t286: F, t14394: F, t14423: F, t14427: F, t14439: F, t14442: F, t14446: F, t14450: F, t14455: F, t285: F, t9614: F) -> (F,) {
    let t288 = 0.0 < t278;
    let t19165 = t19164 * t829;
    let t19166 = t14408 * t19165;
    let t19171 = t14395 * t330;
    let t19173 = t19171 * t19164 * t1003;
    let t19176 = t14401 * t19165;
    let t19180 = piecewise3(t288, t19107, -t19107);
    let t19181 = t1001 * t19180;
    let t19182 = t286 * t19181;
    let t19186 = -t14394 * t19166 / 108.0 + t9614 / 432.0 + t14423 / 216.0 - t14427 + t14439 + t14394 * t19173 / 72.0 + t14394 * t19176 / 72.0 - t285 * t19182 / 96.0 - t14442 - t14446 + t14450 + t14455 / 216.0;
    (t19186,)
}
