//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 745/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk745<F: Float>(t4779: F, t584: F, t9419: F, t20669: F, t20687: F, t1406: F, t6582: F, t9271: F, t10530: F, t6574: F, t6575: F, t2754: F, t874: F) -> (F, F, F, F, F, F, F, F) {
    let t31037 = t584 * t4779 * t9419;
    let t31041 = t584 * t20669;
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31356 = t1406 * t6575;
    let t31585 = t2754 * t874;
    (t31037, t31041, t31047, t31051, t31054, t31119, t31356, t31585)
}
