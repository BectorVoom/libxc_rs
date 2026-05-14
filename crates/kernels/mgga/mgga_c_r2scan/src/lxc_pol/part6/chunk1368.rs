//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1368/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1368<F: Float>(t2169: F, t7310: F, t1632: F, t551: F, t566: F, t7088: F, t2591: F, t481: F, t1568: F, t920: F, t25852: F, t1575: F, t784: F, t546: F, t560: F, t25767: F) -> (F, F, F, F, F, F, F) {
    let t25955 = t2169 * t7310;
    let t25959 = t566 * t551 * t1632 * t7088;
    let t25962 = t2591 * t481;
    let t25963 = t1568 * t920 * t25962;
    let t25964 = t25852 * t25963;
    let t25966 = t1575 * t784;
    let t25967 = t546 * t25966;
    let t25968 = t2591 * t560;
    let t25970 = t25967 * t25767 * t25968;
    (t25955, t25959, t25962, t25963, t25964, t25966, t25970)
}
