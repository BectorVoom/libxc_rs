//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1122/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1122<F: Float>(t20298: F, t20299: F, t6086: F, t1567: F, t489: F, t146: F, t252: F, t1570: F, t481: F, t20242: F, t6535: F, t1553: F, t560: F, t113: F, t6085: F, t6093: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20301 = t20298 * t6086 * t20299;
    let t20303 = t489 * t1567;
    let t20305 = t146 * t20303 * t252;
    let t20306 = t1570 * t481;
    let t20308 = t20305 * t6086 * t20306;
    let t20311 = t6535 * t6086 * t20242;
    let t20313 = t560 * t1553;
    let t20314 = t20313 * t113;
    let t20316 = t6085 * t6086 * t20314;
    let t20318 = t481 * t1553;
    let t20319 = t20318 * t113;
    let t20321 = t6093 * t6086 * t20319;
    (t20301, t20303, t20305, t20306, t20308, t20311, t20313, t20316, t20318, t20319, t20321)
}
