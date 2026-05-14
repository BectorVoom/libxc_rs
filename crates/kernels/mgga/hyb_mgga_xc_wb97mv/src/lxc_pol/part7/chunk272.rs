//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 272/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk272<F: Float>(t7: F, t220: F, t291: F, t770: F, t860: F, t301: F, t644: F, t99: F, t309: F, t298: F, t296: F, t651: F, t98: F, t306: F, t308: F, t313: F, t120: F, t316: F, dens_threshold: F, rho0: F, sigma0: F, tau0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t864 = piecewise3(t9, 0.0, t220 * t860 / 2.0 + t770 * t291 / 2.0);
    let t866 = t301 * t644;
    let t868 = 1.0 / t99 / t866;
    let t869 = t868 * t309;
    let t872 = t298 * sigma0;
    let t873 = t296 * t872;
    let t874 = t301 * t651;
    let t876 = 1.0 / t98 / t874;
    let t878 = 1.0 / t308 / t306;
    let t879 = t876 * t878;
    let t882 = t313 * tau0;
    let t887 = t316 * t120;
    (t864, t868, t869, t872, t873, t876, t878, t879, t882, t887)
}
