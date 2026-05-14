//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1062<F: Float>(t23: F, t287: F, t8291: F, t8294: F, t8297: F, t2474: F, t2534: F, t845: F, t279: F, t5714: F, t1001: F, t3902: F, t999: F, t2363: F, t7262: F, t2368: F, t7263: F) -> (F, F, F, F, F, F) {
    let t24072 = t8291 * t8294 * t23 * t287 * t8297;
    let t24076 = 0.21053604230838734656e2 * t845 * t2474 * t2534;
    let t24088 = 1.0 / t279 / t5714;
    let t24094 = t999 * t3902 * t1001;
    let t24096 = t7262 * t2363;
    let t24099 = t7263 * t2368;
    (t24072, t24076, t24088, t24094, t24096, t24099)
}
