//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 271/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk271<F: Float>(t260: F, t271: F, t784: F, t812: F, t815: F, t820: F, t829: F, t835: F, t839: F, t848: F, t855: F, t857: F) -> (F,) {
    let t860 = -t784 + t812 + t260 * (-0.310907e-1 * t815 * t271 + 1.0 * t820 * t829 + t784 - t812 - 0.19751673498613801407e-1 * t835 + 0.5848223622634646207e0 * t839 * t848) + 0.19751673498613801407e-1 * t260 * t835 - 0.5848223622634646207e0 * t855 * t857;
    (t860,)
}
