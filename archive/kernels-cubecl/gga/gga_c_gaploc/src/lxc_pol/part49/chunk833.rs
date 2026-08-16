//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 833/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk833<F: Float>(t3085: F, t594: F, t4389: F, t899: F, t1415: F, t1397: F, t9297: F, t9290: F, t1457: F, t9424: F, t4779: F, t584: F, t9419: F) -> (F, F, F, F, F, F, F) {
    let t30795 = t594 * t3085;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    let t30839 = t1397 * t9297;
    let t30845 = t1415 * t9290;
    let t30936 = t1457 * t9424;
    let t31037 = t584 * t4779 * t9419;
    (t30795, t30829, t30830, t30839, t30845, t30936, t31037)
}
