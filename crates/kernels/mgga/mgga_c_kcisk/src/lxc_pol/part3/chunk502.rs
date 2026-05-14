//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 502/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk502<F: Float>(t3904: F, t416: F, t140: F, t1429: F, t1434: F, t1460: F, t1470: F, t3560: F, t3566: F, t3588: F, t3594: F, t3620: F, t4244: F, t4253: F, t4264: F, t4266: F, t4269: F, t4274: F, t4279: F, t4284: F, t4288: F, t460: F, t476: F, t479: F) -> (F, F) {
    let t4291 = t416 * t3904;
    let t4295 = 0.619125e-2 * t4244 * t460 + 0.1857375e-1 * t1460 * t1429 - 0.123825e-1 * t1460 * t1434 + 0.46434375e-2 * t476 * t3560 - 0.1857375e-1 * t4253 * t3566 + 0.9286875e-2 * t476 * t3588 + 0.123825e-1 * t476 * t3594 - 0.619125e-2 * t476 * t3620 + t4264 - 0.35374814814814814814e-1 * t4266 - 0.53062222222222222222e-1 * t4269 - 0.44218518518518518518e-1 * t1470 * t4274 - 0.53062222222222222222e-1 * t1470 * t4279 + 0.53062222222222222222e-1 * t1470 * t4284 - 0.26531111111111111111e-1 * t1470 * t4288 - 0.39796666666666666666e-1 * t140 * t479 * t4291;
    (t4291, t4295)
}
