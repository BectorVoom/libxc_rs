//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 648/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk648<F: Float>(t3177: F, t674: F, t1246: F, t151: F, t1242: F, t1852: F, t2063: F, t39: F) -> (F, F, F, F) {
    let t3178 = t3177 * t674;
    let t3182 = t151 * t1246;
    let t3186 = t1852 * t1242;
    let t3188 = t2063 * t39;
    (t3178, t3182, t3186, t3188)
}
