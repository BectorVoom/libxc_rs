//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1319/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1319<F: Float>(t17408: F, t17487: F, t20716: F, t20719: F, t20748: F, t20751: F, t20754: F, t20759: F, t20762: F, t20765: F, t25767: F, t25769: F, t25782: F, t25785: F, t25788: F, t25790: F, t25793: F, t25795: F, t25797: F, t25799: F, t25802: F, t25804: F, t25807: F, t25809: F) -> (F, F) {
    let t26109 = 0.34731666666666666667e0 * t25767 + 0.6311625e0 * t25769 + 0.27545333333333333334e1 * t20716 - 0.103295e1 * t20719 + 0.13892666666666666667e1 * t20748 + 0.13892666666666666667e1 * t20751 - 0.18523555555555555555e1 * t20754 + t17487 + 0.34731666666666666666e0 * t17408 - 0.41678e0 * t20759 - 0.83356e0 * t20762 - 0.41678e0 * t20765;
    let t26122 = -0.6618234375e1 * t25782 + 0.264729375e1 * t25785 + 0.2366859375e0 * t25788 - 0.3529725e1 * t25790 - 0.3529725e1 * t25793 - 0.17648625e1 * t25795 - 0.157790625e0 * t25797 + 0.6311625e0 * t25799 + 0.6311625e0 * t25802 + 0.31558125e0 * t25804 - 0.157790625e0 * t25807 + 0.264729375e1 * t25809;
    (t26109, t26122)
}
