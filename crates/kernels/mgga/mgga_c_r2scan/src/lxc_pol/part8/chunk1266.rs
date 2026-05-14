//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1266/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1266<F: Float>(t2148: F, t29270: F, t6165: F, t481: F, t8832: F, t7614: F, t28404: F, t494: F, t22820: F, t6086: F, t3071: F, t6535: F, t1568: F, t8089: F, t910: F, t6155: F) -> (F, F, F, F, F, F, F, F) {
    let t29272 = t6165 * t2148 * t29270;
    let t29274 = t8832 * t481;
    let t29276 = t7614 * t2148 * t29274;
    let t29279 = t28404 * t494;
    let t29281 = t22820 * t6086 * t29279;
    let t29283 = t3071 * t494;
    let t29285 = t6535 * t6086 * t29283;
    let t29288 = t1568 * t910 * t8089;
    let t29289 = t6155 * t29288;
    (t29272, t29276, t29279, t29281, t29283, t29285, t29288, t29289)
}
