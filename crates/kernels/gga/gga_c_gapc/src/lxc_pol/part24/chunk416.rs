//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 416/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk416<F: Float>(t14: F, t2349: F, t260: F, t445: F, t348: F, t19: F, t269: F, t1355: F, t257: F, t852: F, t4: F, t748: F, t78: F, t1365: F, t854: F, t106: F, t737: F) -> (F, F, F, F, F, F, F) {
    let t2350 = t2349 * t14;
    let t2355 = t260 * t445;
    let t2356 = t2355 * t348;
    let t2357 = t269 * t19;
    let t2358 = t2357 * t1355;
    let t2361 = t14 * t257;
    let t2362 = t852 * t2361;
    let t2364 = t4 * t78 * t748;
    let t2367 = t854 * t1365;
    let t2370 = t106 * t737;
    (t2350, t2356, t2358, t2362, t2364, t2367, t2370)
}
