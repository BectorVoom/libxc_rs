//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1131/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1131<F: Float>(t2195: F, t2597: F, t2294: F, t6139: F, t6140: F, t6118: F, t6123: F, t6148: F, t481: F, t6286: F, t2148: F, t6165: F, t560: F, t6248: F, t7614: F, t1551: F) -> (F, F, F, F, F, F, F, F) {
    let t20499 = t2195 * t2597;
    let t20505 = t6139 * t2294 * t6140;
    let t20507 = t6118 * t6123;
    let t20511 = t2195 * t6148;
    let t20514 = t6286 * t481;
    let t20516 = t6165 * t2148 * t20514;
    let t20518 = t6248 * t560;
    let t20520 = t6165 * t2148 * t20518;
    let t20522 = t6248 * t481;
    let t20524 = t7614 * t2148 * t20522;
    let t20526 = t1551 * t481;
    (t20499, t20505, t20507, t20511, t20516, t20520, t20524, t20526)
}
