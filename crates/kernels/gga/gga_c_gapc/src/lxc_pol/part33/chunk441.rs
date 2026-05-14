//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 441/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk441<F: Float>(t2553: F, t875: F, t2552: F, t122: F, t285: F, t653: F, t277: F, t1087: F, t5: F, t1623: F, t327: F, t186: F, t1631: F, t668: F, t932: F, t2546: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2554 = t2553 * t875;
    let t2555 = t2552 * t2554;
    let t2558 = t285 * t122;
    let t2559 = t2558 * t653;
    let t2560 = t277 * t2559;
    let t2562 = t1087 * t5;
    let t2563 = t1623 * t327 * t2562;
    let t2566 = t285 * t186;
    let t2567 = t2566 * t1631;
    let t2568 = t277 * t2567;
    let t2571 = t932 * t668;
    let t2572 = t2546 * t786;
    (t2554, t2555, t2560, t2562, t2563, t2566, t2568, t2571, t2572)
}
