//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1013/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1013<F: Float>(t40397: F, t40400: F, t42172: F, t42173: F, t42176: F, t42180: F, t42184: F, t42188: F, t42190: F, t42194: F, t42198: F, t48073: F, t47877: F, t587: F, t912: F, t1: F, t47008: F) -> (F, F, F) {
    let t48074 = 0.38342925953920749677e0 * t40397;
    let t48076 = 0.76685851907841499354e0 * t40400;
    let t48078 = t48073 + t42172 + t42173 + t48074 - 0.11502877786176224903e2 * t42176 - t48076 - 0.92023022289409799224e1 * t42180 + t42184 - t42188 + t42190 - t42194 + t42198;
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    (t48078, t48081, t48086)
}
