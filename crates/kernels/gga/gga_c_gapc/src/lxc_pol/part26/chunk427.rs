//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 427/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk427<F: Float>(t2439: F, t2440: F, t640: F, t792: F, t791: F, t1: F, t332: F, t3: F, t875: F, t2416: F, t126: F, t826: F, t824: F, t19: F, t147: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2441 = t2439 * t2440;
    let t2444 = t792 * t640;
    let t2445 = t791 * t2444;
    let t2446 = t332 * t1;
    let t2447 = t3 * t875;
    let t2448 = t2446 * t2447;
    let t2449 = t2416 * t2448;
    let t2452 = t826 * t126;
    let t2453 = t824 * t2452;
    let t2454 = t332 * t19;
    let t2456 = t2454 * t147 * t875;
    (t2441, t2445, t2446, t2448, t2449, t2452, t2453, t2454, t2456)
}
