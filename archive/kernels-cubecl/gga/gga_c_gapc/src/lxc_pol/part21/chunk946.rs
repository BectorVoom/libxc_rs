//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 946/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk946<F: Float>(t11534: F, t11558: F, t1026: F, t632: F, t3018: F, t3022: F, t3691: F, t3679: F, t5248: F, t1643: F, t3683: F, t424: F) -> (F, F, F, F, F, F, F) {
    let t11559 = t11534 * t11558;
    let t11561 = t632 * t1026;
    let t11562 = t11561 * t3018;
    let t11564 = t3691 * t3022;
    let t11566 = t3679 * t5248;
    let t11567 = t1643 * t11566;
    let t11569 = t424 * t3683;
    (t11559, t11561, t11562, t11564, t11566, t11567, t11569)
}
