//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1047/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1047<F: Float>(t11549: F, t8751: F, t11399: F, t5700: F, t1678: F, t11397: F, t632: F, t11533: F, t424: F, t3703: F, t1667: F, t11398: F, t3691: F, t9099: F, t11566: F, t5252: F) -> (F, F, F, F, F, F, F) {
    let t34330 = t11549 * t8751;
    let t34333 = t11399 * t5700;
    let t34334 = t34333 * t1678;
    let t34335 = t632 * t11397 * t34334;
    let t34337 = t424 * t11533;
    let t34338 = t34337 * t3703;
    let t34340 = t34333 * t1667;
    let t34341 = t11398 * t34340;
    let t34344 = t3691 * t9099;
    let t34346 = t5252 * t11566;
    (t34330, t34335, t34337, t34338, t34341, t34344, t34346)
}
