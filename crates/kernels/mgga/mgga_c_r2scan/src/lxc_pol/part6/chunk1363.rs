//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1363/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1363<F: Float>(t19875: F, t25214: F, t545: F, t2567: F, t264: F, t133: F, t1604: F, t1605: F, t7088: F, t2185: F, t2148: F, t22790: F, t2599: F, t3433: F, t1575: F, t2102: F, t571: F) -> (F, F, F, F, F) {
    let t25804 = t545 * t19875 * t25214;
    let t25805 = t264 * t2567;
    let t25811 = t1604 * t1605 * t133 * t7088;
    let t25813 = t2567 * t2185;
    let t25815 = t22790 * t2148 * t25813;
    let t25826 = t3433 * t2599;
    let t25827 = t571 * t1575 * t2102 * t25826;
    (t25804, t25805, t25811, t25815, t25827)
}
