//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 755/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk755<F: Float>(t20669: F, t584: F, t20687: F, t1406: F, t6582: F, t9271: F, t10530: F, t6574: F, t123: F, t18313: F, t197: F, t3116: F) -> (F, F, F, F, F, F, F) {
    let t31041 = t584 * t20669;
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31120 = t18313 * t123;
    let t31139 = t197 * t3116;
    (t31041, t31047, t31051, t31054, t31119, t31120, t31139)
}
