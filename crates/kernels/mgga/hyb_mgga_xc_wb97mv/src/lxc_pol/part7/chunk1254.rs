//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1254/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1254<F: Float>(t7: F, t1874: F, t29125: F, t10273: F, t10965: F, t10970: F, t1861: F, t1877: F, t2181: F, t22450: F, t26329: F, t2708: F, t3319: F, t3854: F, t3864: F, t453: F, t544: F, t6175: F, t6768: F, t8913: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t30755 = t29125 * t1874;
    let t30776 = piecewise3(t8, 0.0, 280.0 / 81.0 * t22450 * t3864 * t1861 - 224.0 / 27.0 * t8913 * t30755 - 28.0 / 27.0 * t10965 * t1877 + 32.0 / 9.0 * t2181 * t453 * t2708 + 16.0 / 9.0 * t3319 * t1874 - 16.0 / 3.0 * t3319 * t6175 - 28.0 / 27.0 * t6768 * t3854 * t1861 + 8.0 / 9.0 * t2181 * t10273 * t544 + 4.0 / 9.0 * t10970 * t1877 - t26329);
    (t30755, t30776)
}
