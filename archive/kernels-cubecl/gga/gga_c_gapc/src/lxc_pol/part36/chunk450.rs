//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 450/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk450<F: Float>(t2468: F, t319: F, t972: F, t195: F, t896: F, t311: F, t668: F, t761: F, t285: F, t5: F, t1033: F, t277: F) -> (F, F, F, F, F, F) {
    let t2469 = t319 * t2468;
    let t2470 = t972 * t972;
    let t2473 = t896 * t195;
    let t2474 = t311 * t2473;
    let t2477 = t761 * t668;
    let t2480 = t285 * t5;
    let t2481 = t2480 * t1033;
    let t2482 = t277 * t2481;
    (t2469, t2470, t2474, t2477, t2480, t2482)
}
