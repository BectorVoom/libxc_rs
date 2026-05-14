//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 784/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk784<F: Float>(t3972: F, t430: F, t1302: F, t418: F, t195: F, t1479: F, t1486: F, t328: F, t407: F, t1618: F, t414: F, t423: F, t1491: F, t1502: F, t409: F, t429: F, tau1: F) -> (F, F, F, F, F, F) {
    let t4419 = t430 * t3972;
    let t4423 = t418 * t1302;
    let t4425 = 1.0 / t195 / t4423;
    let t4426 = t1479 * t4425;
    let t4427 = t1486 * tau1;
    let t4428 = t4426 * t4427;
    let t4431 = t328 * t407;
    let t4432 = t4431 * t430;
    let t4433 = t418 * t1618;
    let t4435 = 1.0 / t195 / t4433;
    let t4436 = t414 * t4435;
    let t4437 = t423 * t3972;
    let t4438 = t4436 * t4437;
    let t4441 = t1491 * t1502;
    let t4447 = 1.0 / t429 / t409;
    (t4419, t4428, t4432, t4438, t4441, t4447)
}
