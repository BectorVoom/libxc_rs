//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1141/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1141<F: Float>(t23171: F, t23177: F, t2554: F, t363: F, t2557: F, t2565: F, t2572: F, t7258: F, t982: F, t2526: F, t2533: F, t7332: F, t963: F, t2697: F, t7536: F, t1026: F, t1037: F, t2631: F, t2659: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23413 = 0.16979925925925925926e1 * t23171;
    let t23415 = 0.5356037037037037037e1 * t23177;
    let t23436 = t2554 * t2554;
    let t23438 = t363 / t23436;
    let t23439 = t2557 * t2557;
    let t23440 = 1.0 / t23439;
    let t23447 = t2565 * t2572;
    let t23450 = t982 * t7258;
    let t23456 = t2526 * t2533;
    let t23459 = t963 * t7332;
    let t23522 = 0.17757530864197530864e0 * t23177;
    let t23542 = t2697 * t7536;
    let t23547 = 0.4274e0 * t1026 * t2631 * t2659 * t1037;
    (t23413, t23415, t23438, t23440, t23447, t23450, t23456, t23459, t23522, t23542, t23547)
}
