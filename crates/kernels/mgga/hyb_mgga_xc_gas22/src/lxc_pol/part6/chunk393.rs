//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 393/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk393<F: Float>(t1483: F, t1486: F, t209: F, t328: F, t407: F, t430: F, t194: F, t418: F, t196: F, t414: F, t423: F, t211: F, t429: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t1487 = t1483 * t1486;
    let t1491 = t328 * t407 * t209;
    let t1492 = t1491 * t430;
    let t1493 = t418 * t194;
    let t1495 = F::cast_from(1.0_f64) / t196 / t1493;
    let t1496 = t414 * t1495;
    let t1497 = t423 * tau1;
    let t1498 = t1496 * t1497;
    let t1502 = F::cast_from(1.0_f64) / t429 / t211;
    (t1487, t1491, t1492, t1493, t1497, t1498, t1502)
}
