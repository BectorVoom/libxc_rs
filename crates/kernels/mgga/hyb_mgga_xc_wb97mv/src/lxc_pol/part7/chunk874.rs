//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 874/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk874<F: Float>(t7501: F, t7504: F, t7507: F, t7510: F, t7514: F, t7516: F, t7518: F, t7521: F, t1070: F, t1036: F, t7584: F, t2665: F, t7599: F, t222: F, t2660: F, t2753: F) -> (F, F, F, F, F, F) {
    let t7633 = -0.47063e1 * t7501 + 0.31375333333333333334e1 * t7504 - 0.36604555555555555556e1 * t7507 - 0.16068111111111111111e1 * t7510 + 0.28051666666666666666e0 * t7514 - 0.56103333333333333332e0 * t7516 - 0.6545388888888888889e0 * t7518 - 0.46308888888888888888e0 * t7521;
    let t7634 = t7633 * t1070;
    let t7637 = t7584 * t1036;
    let t7639 = 6.0 * t2665 * t7637;
    let t7640 = t7599 * t1070;
    let t7645 = 0.53424999999999999999e-1 * t222 * t2753 * t2660;
    (t7633, t7634, t7637, t7639, t7640, t7645)
}
