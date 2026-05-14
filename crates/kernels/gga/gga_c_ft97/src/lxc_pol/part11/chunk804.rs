//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 804/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk804<F: Float>(t38262: F, t38264: F, t446: F, t1586: F, t1642: F, t1588: F, t1643: F, t432: F, t7959: F, t7793: F, t1651: F, t7824: F, t1882: F, t7816: F, t1647: F, t1755: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38266 = t446 * t38262 * t38264;
    let t38268 = t1642 * t1586;
    let t38269 = t1643 * t1588;
    let t38271 = t446 * t38268 * t38269;
    let t38273 = t7959 * t432;
    let t38275 = t446 * t7793 * t38273;
    let t38277 = t1651 * t1588;
    let t38279 = t446 * t7824 * t38277;
    let t38281 = t1882 * t7816;
    let t38283 = t1647 * t1755;
    (t38266, t38269, t38271, t38273, t38275, t38277, t38279, t38281, t38283)
}
