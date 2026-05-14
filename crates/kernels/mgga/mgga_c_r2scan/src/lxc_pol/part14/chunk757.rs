//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 757/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk757<F: Float>(t113: F, t7194: F, t2530: F, t494: F, t1550: F, t920: F, t1553: F, t3270: F, t792: F, t1561: F, t983: F, t2847: F, t498: F, t6343: F, t910: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7195 = t7194 * t113;
    let t7197 = t2530 * t494;
    let t7202 = t920 * t1550;
    let t7204 = t920 * t1553;
    let t7206 = t3270 * t792;
    let t7217 = t1561 * t983;
    let t7218 = t7217 * t792;
    let t7221 = t498 * t2847;
    let t7233 = t6343 * t910;
    let t7234 = t551 * t7233;
    (t7195, t7197, t7202, t7204, t7206, t7218, t7221, t7233, t7234)
}
