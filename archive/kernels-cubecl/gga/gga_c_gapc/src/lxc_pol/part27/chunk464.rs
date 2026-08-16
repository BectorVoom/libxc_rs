//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 464/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk464<F: Float>(t1631: F, t2566: F, t277: F, t668: F, t932: F, t2546: F, t786: F, t2552: F, t122: F, t125: F, t2206: F, t311: F) -> (F, F, F, F, F, F) {
    let t2567 = t2566 * t1631;
    let t2568 = t277 * t2567;
    let t2571 = t932 * t668;
    let t2572 = t2546 * t786;
    let t2573 = t2552 * t2572;
    let t2577 = t2206 * t122 * t125;
    let t2578 = t311 * t2577;
    (t2568, t2571, t2572, t2573, t2577, t2578)
}
