//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1419/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1419<F: Float>(t22491: F, t21969: F, t21972: F, t22473: F, t22478: F, t22481: F, t22484: F, t22487: F, t22489: F, t22496: F, t22500: F, t22501: F, t22505: F, t1734: F, t7829: F, t1726: F, t1727: F, t2782: F) -> (F, F, F) {
    let t26831 = 480.0 * t22491;
    let t26834 = -0.13012297560362087811e0 * t22473 - t22478 - t22481 + t22484 - t22487 - 3.0 * t22489 - t26831 - t21969 - t21972 - t22496 - t22500 - 0.35089341735807877242e1 * t22501 - 0.35089341735807877242e1 * t22505;
    let t26835 = t7829 * t1734;
    let t26838 = t1726 * t2782 * t1727;
    (t26834, t26835, t26838)
}
