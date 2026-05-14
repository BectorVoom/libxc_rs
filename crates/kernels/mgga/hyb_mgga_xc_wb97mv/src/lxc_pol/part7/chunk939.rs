//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 939/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk939<F: Float>(t7: F, t132: F, t8557: F, t8593: F, t8629: F, t8879: F, t1874: F, t6175: F, t2322: F, t3435: F, t3461: F, t1323: F, t1847: F, t222: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t8882 = piecewise3(t134, 0.0, t8557 + t8593 + t8629 + t8879);
    let t8883 = 2.0 * t1874;
    let t8884 = 6.0 * t6175;
    let t8885 = t8883 - t8884;
    let t8886 = piecewise3(t8, 0.0, t8885);
    let t8901 = t2322 * t3435;
    let t8902 = t8901 * t3461;
    let t8908 = t222 * t1847 * t1323;
    (t8882, t8883, t8884, t8885, t8886, t8902, t8908)
}
