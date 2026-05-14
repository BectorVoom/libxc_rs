//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1195/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1195<F: Float>(t2735: F, t500: F, t1878: F, t218: F, t2774: F, t2778: F, t1079: F, t5555: F, t675: F, t7391: F, t7395: F, t7399: F, t1107: F, t5838: F, t5845: F, t1854: F, t2743: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20741 = 8.0 * t2735 * t500;
    let t20748 = t218 * t1878 * t2774;
    let t20751 = t218 * t1878 * t2778;
    let t20754 = t218 * t5555 * t1079;
    let t20759 = t218 * t675 * t7391;
    let t20762 = t218 * t675 * t7395;
    let t20765 = t218 * t675 * t7399;
    let t20834 = t5838 * t1107;
    let t20837 = t5845 * t1107;
    let t20893 = t2743 * t1854;
    (t20741, t20748, t20751, t20754, t20759, t20762, t20765, t20834, t20837, t20893)
}
