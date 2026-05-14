//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1030/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1030<F: Float>(t1397: F, t6603: F, t9287: F, t1415: F, t6699: F, t7030: F, t20358: F, t2365: F, t7025: F, t20731: F, t544: F, t2371: F, t4398: F, t20670: F, t20671: F, t20696: F) -> (F, F, F, F, F, F, F, F) {
    let t30642 = t1397 * t6603;
    let t30644 = 0.59584149919750711116e-1 * t30642 * t9287;
    let t30647 = 0.59584149919750711116e-1 * t1415 * t6699 * t7030;
    let t30650 = 0.29792074959875355558e-1 * t7025 * t2365 * t20358;
    let t30703 = t544 * t20731;
    let t30705 = 0.59584149919750711116e-1 * t30703 * t9287;
    let t30708 = 0.59584149919750711116e-1 * t4398 * t2371 * t7030;
    let t30712 = 0.17041300423964777634e0 * t20670 * t20671 * t20696;
    (t30642, t30644, t30647, t30650, t30703, t30705, t30708, t30712)
}
