//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 738/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk738<F: Float>(t20687: F, t584: F, t1406: F, t6582: F, t9271: F, t10530: F, t6574: F, t123: F, t18313: F, t197: F, t3116: F, t1397: F, t9301: F, t30208: F, t493: F, t1339: F, t29969: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31120 = t18313 * t123;
    let t31139 = t197 * t3116;
    let t31182 = t1397 * t9301;
    let t31300 = t493 * t30208;
    let t31308 = t1339 * t29969;
    (t31047, t31051, t31054, t31119, t31120, t31139, t31182, t31300, t31308)
}
