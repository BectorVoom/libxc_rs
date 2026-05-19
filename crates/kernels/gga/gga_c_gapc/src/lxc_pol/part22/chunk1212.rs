//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1212/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1212<F: Float>(t34337: F, t3703: F, t1667: F, t34333: F, t11398: F, t3691: F, t9099: F, t11566: F, t5252: F, t128: F, t1643: F, t5248: F, t671: F) -> (F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t34338 = t34337 * t3703;
    let t34340 = t34333 * t1667;
    let t34341 = t11398 * t34340;
    let t34344 = t3691 * t9099;
    let t34346 = t5252 * t11566;
    let t34351 = t1643 * t128 * t671 * pi * t5248;
    (t34338, t34341, t34344, t34346, t34351)
}
