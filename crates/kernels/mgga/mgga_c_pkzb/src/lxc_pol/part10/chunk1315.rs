//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1315/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1315<F: Float>(t25633: F, t25636: F, t25639: F, t25734: F, t25737: F, t25740: F, t25744: F, t25747: F, t25750: F, t25754: F, t25758: F, t25762: F, t17403: F, t17408: F, t20716: F, t20719: F, t20748: F, t20751: F, t20754: F, t20759: F, t20762: F, t20765: F, t25767: F, t25769: F) -> (F, F) {
    let t25995 = 0.40256666666666666667e0 * t25633 - 0.60385e0 * t25636 + 0.905775e0 * t25639 + 0.27595e0 * t25734 + 0.49671e0 * t25737 - 0.66228e0 * t25740 + 0.49671e0 * t25744 - 0.33114e0 * t25747 - 0.33114e0 * t25750 + 0.248355e0 * t25754 + 0.49671e0 * t25758 + 0.248355e0 * t25762;
    let t26008 = 0.27595e0 * t25767 + 0.16504875e0 * t25769 + 0.16102666666666666667e1 * t20716 - 0.60385e0 * t20719 + 0.11038e1 * t20748 + 0.11038e1 * t20751 - 0.14717333333333333333e1 * t20754 + t17403 + 0.27595e0 * t17408 - 0.33114e0 * t20759 - 0.66228e0 * t20762 - 0.33114e0 * t20765;
    (t25995, t26008)
}
