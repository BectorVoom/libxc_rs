//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1302/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1302<F: Float>(t1878: F, t218: F, t3546: F, t25714: F, t672: F, t17408: F, t17548: F, t20716: F, t20719: F, t20748: F, t20751: F, t20754: F, t20759: F, t20762: F, t20765: F, t17432: F, t1862: F, t3528: F) -> (F, F, F, F) {
    let t25767 = t218 * t1878 * t3546;
    let t25769 = t672 * t25714;
    let t25780 = 0.27385555555555555555e0 * t25767 + 0.3071625e0 * t25769 + 0.15944888888888888889e1 * t20716 - 0.59793333333333333334e0 * t20719 + 0.10954222222222222222e1 * t20748 + 0.10954222222222222222e1 * t20751 - 0.14605629629629629629e1 * t20754 + t17548 + 0.27385555555555555556e0 * t17408 - 0.32862666666666666666e0 * t20759 - 0.65725333333333333332e0 * t20762 - 0.32862666666666666666e0 * t20765;
    let t25782 = t17432 * t3528 * t1862;
    (t25767, t25769, t25780, t25782)
}
