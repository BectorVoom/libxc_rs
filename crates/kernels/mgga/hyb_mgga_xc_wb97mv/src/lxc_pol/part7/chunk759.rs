//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 759/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk759<F: Float>(t7: F, t1232: F, t3177: F, t3299: F, t3988: F, t687: F, t3979: F, t3854: F, t2181: F, t3864: F, t775: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t4122 = t3177 * t1232;
    let t4126 = t3299 * t1232;
    let t4130 = t687 * t3988;
    let t4134 = t687 * t3979;
    let t4143 = piecewise3(t8, 0.0, t3854);
    let t4153 = piecewise3(t8, 0.0, 4.0 / 9.0 * t2181 * t3864 - t775 * t3854 / 3.0);
    (t4122, t4126, t4130, t4134, t4143, t4153)
}
