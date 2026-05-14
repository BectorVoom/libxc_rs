//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 909/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk909<F: Float>(t3046: F, t54: F, t57: F, t60: F, t63: F, t1187: F, t6238: F, t1190: F, t1192: F, t1194: F, t1196: F, t1198: F, t1200: F, t1202: F, t1842: F, t1883: F, t3049: F, t3054: F, t3059: F, t3064: F, t583: F) -> (F, F) {
    let t8305 = t54 * t3046;
    let t8310 = t57 * t3046;
    let t8315 = t60 * t3046;
    let t8320 = t63 * t3046;
    let t8339 = t6238 * t1187;
    let t8342 = -t8305 * t583 / 24.0 - t3049 * t1883 / 48.0 + t8310 * t583 / 320.0 + t3054 * t1883 / 640.0 - t8315 * t583 / 5760.0 - t3059 * t1883 / 11520.0 + t8320 * t583 / 129024.0 + t3064 * t1883 / 258048.0 - 2.0 / 3.0 * t1190 * t1842 + t1192 * t1842 / 8.0 - t1194 * t1842 / 80.0 + t1196 * t1842 / 1152.0 - t1198 * t1842 / 21504.0 + t1200 * t1842 / 491520.0 - t1202 * t1842 / 13271040.0 + t8339 * t1842 / 412876800.0;
    (t8339, t8342)
}
