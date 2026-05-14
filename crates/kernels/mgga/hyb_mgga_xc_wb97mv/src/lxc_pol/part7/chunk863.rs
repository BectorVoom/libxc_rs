//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 863/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk863<F: Float>(t2518: F, t358: F, t2572: F, t982: F, t2473: F, t933: F, t2533: F, t963: F, t2565: F, t260: F, t1057: F, t2715: F, t1041: F, t2776: F, t1037: F, t2632: F, t2659: F) -> (F, F, F, F, F, F, F, F) {
    let t7405 = 1.0 / t2518 / t358;
    let t7409 = t982 * t2572;
    let t7415 = t933 * t2473;
    let t7421 = t963 * t2533;
    let t7434 = t260 * t2565;
    let t7473 = t2715 * t1057;
    let t7475 = t1041 * t2776;
    let t7479 = 6.0 * t2632 * t1037 * t2659;
    (t7405, t7409, t7415, t7421, t7434, t7473, t7475, t7479)
}
