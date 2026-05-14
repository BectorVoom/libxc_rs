//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 883/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk883<F: Float>(t1142: F, t2849: F, t2869: F, t2873: F, t1122: F, t1126: F, sigma0: F) -> (F, F, F, F, F) {
    let t7819 = t1142 * t2849;
    let t7823 = t1142 * t2869;
    let t7827 = t1142 * t2873;
    let t7831 = t1122 * sigma0;
    let t7832 = t1126 * t7831;
    (t7819, t7823, t7827, t7831, t7832)
}
