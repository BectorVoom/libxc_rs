//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 286/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk286<F: Float>(t2389: F, t898: F, t1457: F, t2345: F, t1445: F, t2335: F, t2344: F, t447: F, t528: F, t894: F, t1: F, t874: F, t106: F, t192: F, t524: F, t529: F) -> (F, F, F, F, F, F, F) {
    let t2390 = t898 * t2389;
    let t2392 = t1457 * t2345;
    let t2395 = t1445 * t2335;
    let t2398 = t2344 * t447;
    let t2399 = t1445 * t2398;
    let t2402 = t528 * t894;
    let t2405 = t874 * t1;
    let t2406 = t2405 * t106;
    let t2407 = t2406 * t192;
    let t2410 = t524 * t529;
    (t2390, t2392, t2395, t2399, t2402, t2407, t2410)
}
