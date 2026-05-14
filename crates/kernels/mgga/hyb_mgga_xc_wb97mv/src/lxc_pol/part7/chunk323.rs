//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 323/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk323<F: Float>(t1099: F, t1101: F, t1012: F, t1017: F, t1039: F, t1043: F, t1048: F, t1056: F, t1057: F, t1091: F, t1098: F, t458: F, t489: F, t864: F, t101: F, tau0: F) -> (F, F, F) {
    let t1103 = 0.5848223622634646207e0 * t1099 * t1101;
    let t1104 = t1017 + t1039 + t1043 - t1048 + t458 * t1057 + t1091 + 0.19751673498613801407e-1 * t1056 * t489 - t1098 - t1103 - t864 - t1012;
    let t1106 = tau0 * t101;
    (t1103, t1104, t1106)
}
