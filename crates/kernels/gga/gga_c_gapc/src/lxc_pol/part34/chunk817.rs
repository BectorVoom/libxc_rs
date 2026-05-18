//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 817/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk817<F: Float>(t3132: F, t5395: F, t5392: F, t3128: F, t5633: F, t3133: F, t633: F, t8992: F, t1835: F, t1691: F, t129: F, t4948: F) -> (F, F, F, F, F, F) {
    let t9336 = t5395 * t3132;
    let t9337 = t9336 * t5392;
    let t9339 = t3128 * t5633;
    let t9341 = t3133 * t5633;
    let t9343 = t633 * t8992;
    let t9344 = t9343 * t1835;
    let t9346 = t9343 * t1691;
    let t9348 = t4948 * t129;
    (t9337, t9339, t9341, t9344, t9346, t9348)
}
