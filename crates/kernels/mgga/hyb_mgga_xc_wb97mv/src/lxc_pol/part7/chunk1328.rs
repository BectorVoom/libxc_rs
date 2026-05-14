//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1328/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1328<F: Float>(t7: F, t10273: F, t11586: F, t11591: F, t1861: F, t1874: F, t1877: F, t24042: F, t2708: F, t27719: F, t2791: F, t30755: F, t3641: F, t3854: F, t3864: F, t453: F, t544: F, t6175: F, t7710: F, t9656: F, zeta_threshold: F) -> (F,) {
    let t8 = t7 <= zeta_threshold;
    let t32476 = piecewise3(t8, 0.0, 40.0 / 81.0 * t24042 * t3864 * t1861 - 64.0 / 27.0 * t9656 * t30755 - 8.0 / 27.0 * t11586 * t1877 + 32.0 / 9.0 * t2791 * t453 * t2708 + 16.0 / 9.0 * t3641 * t1874 - 16.0 / 3.0 * t3641 * t6175 - 8.0 / 27.0 * t7710 * t3854 * t1861 + 8.0 / 9.0 * t2791 * t10273 * t544 + 4.0 / 9.0 * t11591 * t1877 + t27719);
    (t32476,)
}
