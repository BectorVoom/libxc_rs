//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1210/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1210<F: Float>(t10284: F, t1993: F, t6461: F, t10303: F, t10306: F, t10395: F, t10400: F, t10403: F, t10408: F, t10411: F, t10416: F, t10419: F, t10424: F, t10427: F, t1842: F, t1883: F, t21484: F, t3054: F, t3059: F, t3064: F, t3069: F, t3074: F, t3877: F, t3894: F, t6238: F, t8267: F) -> (F, F) {
    let t29160 = t1993 * t6461 * t10284;
    let t29200 = -19.0 / 412876800.0 * t21484 * t3877 * t1842 + t6238 * t3894 * t1842 / 412876800.0 - t10395 * t1883 / 80.0 + t3054 * t8267 / 320.0 + t10400 * t1883 / 640.0 + t10403 * t1883 / 1152.0 - t3059 * t8267 / 5760.0 - t10408 * t1883 / 11520.0 - t10411 * t1883 / 21504.0 + t3064 * t8267 / 129024.0 + t10416 * t1883 / 258048.0 + t10419 * t1883 / 491520.0 - t3069 * t8267 / 3440640.0 - t10424 * t1883 / 6881280.0 - t10427 * t1883 / 13271040.0 + t3074 * t8267 / 0.10616832e9 + t10303 * t1883 / 0.21233664e9 + t10306 * t1883 / 412876800.0;
    (t29160, t29200)
}
