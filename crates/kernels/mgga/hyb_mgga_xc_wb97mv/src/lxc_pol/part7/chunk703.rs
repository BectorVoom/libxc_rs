//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 703/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk703<F: Float>(t7: F, t132: F, t1101: F, t3638: F, t1173: F, t2791: F, t224: F, t3: F, t1874: F, t544: F, t1232: F, t2799: F, t339: F, t674: F, t259: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t3639 = t3638 * t1101;
    let t3641 = t2791 * t1173;
    let t3644 = t224 * t3;
    let t3648 = piecewise3(t8, 0.0, 4.0 / 9.0 * t3641 * t544 + 8.0 / 3.0 * t3644 * t1874);
    let t3649 = t2799 * t1232;
    let t3652 = t339 * t3;
    let t3656 = piecewise3(t133, 0.0, 4.0 / 9.0 * t3649 * t674 - 8.0 / 3.0 * t3652 * t1874);
    let t3658 = (t3648 + t3656) * t259;
    (t3639, t3641, t3644, t3649, t3652, t3658)
}
