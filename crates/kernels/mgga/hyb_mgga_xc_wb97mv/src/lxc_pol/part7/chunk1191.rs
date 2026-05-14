//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1191/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1191<F: Float>(t1399: F, t238: F, t6812: F, t800: F, t9346: F, t9351: F, t9355: F, t2595: F, t260: F, t9395: F, t7316: F, t1403: F, t7403: F, t1420: F, t7258: F, t1408: F, t7332: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27159 = t238 * t6812 * t1399;
    let t27207 = t238 * t800 * t9346;
    let t27210 = t238 * t800 * t9351;
    let t27213 = t238 * t800 * t9355;
    let t27242 = t260 * t2595;
    let t27346 = t260 * t9395;
    let t27359 = t260 * t7316;
    let t27384 = t7403 * t1403;
    let t27393 = t1420 * t7258;
    let t27396 = t1408 * t7332;
    (t27159, t27207, t27210, t27213, t27242, t27346, t27359, t27384, t27393, t27396)
}
