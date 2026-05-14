//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1252/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1252<F: Float>(t7: F, t132: F, t29907: F, t29964: F, t30014: F, t30069: F, t30106: F, t30150: F, t30197: F, t30723: F, t26273: F, t1847: F, t222: F, t4153: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t30727 = piecewise3(t134, 0.0, t29907 + t29964 + t30014 + t30069 + t30106 + t30150 + t30197 + t30723);
    let t30728 = piecewise3(t8, 0.0, t26273);
    let t30747 = t222 * t1847 * t4153;
    (t30727, t30728, t30747)
}
