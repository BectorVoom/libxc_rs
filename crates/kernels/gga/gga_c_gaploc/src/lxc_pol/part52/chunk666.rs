//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 666/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk666<F: Float>(t122: F, t2310: F, t481: F, t4260: F, t883: F, t2321: F, t28438: F, t4389: F, t899: F, t1415: F, t4779: F, t584: F, t9419: F, t20669: F, t20687: F, t1406: F, t6582: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29874 = t481 * t2310 * t122;
    let t30204 = t4260 * t883;
    let t30733 = t28438 * t2321;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    let t31037 = t584 * t4779 * t9419;
    let t31041 = t584 * t20669;
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    (t29874, t30204, t30733, t30829, t30830, t31037, t31041, t31047, t31051)
}
