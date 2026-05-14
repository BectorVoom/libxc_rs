//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1006/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1006<F: Float>(t3809: F, t7833: F, t2856: F, t516: F, t3813: F, t1157: F, t7817: F, sigma0: F) -> (F, F, F, F, F) {
    let t10061 = t7833 * t3809;
    let t10064 = t2856 * sigma0;
    let t10065 = t516 * t10064;
    let t10066 = t7833 * t3813;
    let t10069 = t1157 * t7817;
    (t10061, t10064, t10065, t10066, t10069)
}
