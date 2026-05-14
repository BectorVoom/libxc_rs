//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1184/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1184<F: Float>(t1630: F, t1632: F, t938: F, t100954: F, t101279: F, t101282: F, t101285: F, t101295: F, t11209: F, t22513: F, t22603: F, t22761: F, t22767: F, t22819: F, t25626: F, t25708: F, t25787: F, t3099: F, t53: F, t5538: F, t5540: F, t5591: F, t6450: F, t72: F, t7982: F, t92327: F, t93026: F) -> (F, F) {
    let t101298 = t938 * t1630 * t1632;
    let t101312 = -0.68099848938271604939e-1 * t25708 * t101279 + 0.17659850543899795696e-2 * t22513 * t101282 - 0.13519760450715832853e-3 * t7982 * t101285 - 0.67552196935353456646e-5 * t11209 * t25626 + 0.74233839446572641111e-4 * t93026 + 0.61289864044444444444e0 * t22761 * t22767 * t25787 - 0.76612330055555555555e-1 * t101295 + 0.51690243689028715488e-5 * t22603 * t5540 * t101298 + 0.62028292426834458586e-5 * t5538 * t5540 * t100954 - 0.12768721675925925926e-1 * t92327 * t6450 + 0.18164417702296932716e-2 * t22819 * t5591 * t72 * t3099 * t53;
    (t101298, t101312)
}
