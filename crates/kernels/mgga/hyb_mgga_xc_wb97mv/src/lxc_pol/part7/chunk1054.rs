//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1054/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1054<F: Float>(t7: F, t132: F, t10587: F, t10851: F, t10891: F, t10903: F, t10273: F, t4229: F, t6981: F, t3461: F, t3338: F, t9127: F, t3374: F, t9120: F, t4163: F, t808: F, t2245: F, t1341: F, t3369: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t10906 = piecewise3(t134, 0.0, t10587 + t10851 + t10891 + t10903);
    let t10907 = piecewise3(t8, 0.0, t10273);
    let t10916 = t6981 * t4229;
    let t10917 = t10916 * t3461;
    let t10921 = 4.0 * t9127 * t3338;
    let t10923 = 0.32163958997385070134e2 * t9120 * t3374;
    let t10924 = t4163 * t808;
    let t10926 = 6.0 * t2245 * t10924;
    let t10927 = t1341 * t3369;
    (t10906, t10907, t10916, t10917, t10921, t10923, t10924, t10926, t10927)
}
