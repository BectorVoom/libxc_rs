//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 765/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk765<F: Float>(t222: F, t37: F, t4234: F, t2455: F, t3461: F, t361: F, t1410: F, t3477: F, t1409: F, t968: F) -> (F, F, F, F, F, F) {
    let t4236 = t222 * t37 * t4234;
    let t4238 = t2455 - 0.35616666666666666666e-1 * t3461 + 0.53425e-1 * t4236;
    let t4240 = 0.621814e-1 * t4238 * t361;
    let t4242 = 2.0 * t3477 * t1410;
    let t4243 = t1409 * t1409;
    let t4244 = t4243 * t968;
    (t4236, t4238, t4240, t4242, t4243, t4244)
}
