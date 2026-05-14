//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 884/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk884<F: Float>(t133: F, t1543: F, t6243: F, t1604: F, t1234: F, t277: F, t495: F, t360: F, t1554: F, t2134: F, t119: F, t6100: F, t122: F, t507: F, t162: F, t500: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6245 = t6243 * t133 * t1543;
    let t6246 = t1604 * t6245;
    let t6248 = t277 * t1234;
    let t6249 = t6248 * t495;
    let t6250 = t360 * t6249;
    let t6253 = t2134 * t1554;
    let t6254 = t360 * t6253;
    let t6257 = t6100 * t119;
    let t6260 = 0.98171973930797904389e-1 * t6257 * t122 * t507;
    let t6261 = t162 * t500;
    (t6245, t6246, t6248, t6249, t6250, t6253, t6254, t6257, t6260, t6261)
}
