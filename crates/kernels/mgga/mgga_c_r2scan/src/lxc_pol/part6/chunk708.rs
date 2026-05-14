//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 708/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk708<F: Float>(t4741: F, t1736: F, t3: F, t40: F, t1401: F, t4735: F, t392: F, t4738: F, t22: F, t502: F, t6: F, t4733: F, t4736: F, t4739: F, t438: F, t1449: F, t430: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4742 = 0.16068111111111111111e1 * t4741;
    let t4743 = t1736 * t3;
    let t4744 = t4743 * t40;
    let t4745 = 0.28051666666666666666e0 * t4744;
    let t4746 = t1401 * t4735;
    let t4747 = 0.56103333333333333332e0 * t4746;
    let t4748 = t392 * t4738;
    let t4749 = 0.6545388888888888889e0 * t4748;
    let t4751 = t22 * t6 * t502;
    let t4752 = 0.46308888888888888888e0 * t4751;
    let t4753 = -0.47063e1 * t4733 + 0.31375333333333333334e1 * t4736 - 0.36604555555555555556e1 * t4739 - t4742 + t4745 - t4747 - t4749 - t4752;
    let t4754 = t4753 * t438;
    let t4758 = 1.0 / t1449 / t430;
    (t4742, t4743, t4744, t4745, t4746, t4747, t4748, t4749, t4751, t4752, t4753, t4754, t4758)
}
