//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 799/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk799<F: Float>(t7: F, t132: F, t4444: F, t4491: F, t224: F, t2791: F, t3854: F, t3864: F, t2799: F, t339: F, t3979: F, t3988: F, t259: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t4492 = t4444 + t4491;
    let t4501 = piecewise3(t8, 0.0, 4.0 / 9.0 * t2791 * t3864 + 4.0 / 3.0 * t224 * t3854);
    let t4507 = piecewise3(t133, 0.0, 4.0 / 9.0 * t2799 * t3979 + 4.0 / 3.0 * t339 * t3988);
    let t4509 = (t4501 + t4507) * t259;
    (t4492, t4509)
}
