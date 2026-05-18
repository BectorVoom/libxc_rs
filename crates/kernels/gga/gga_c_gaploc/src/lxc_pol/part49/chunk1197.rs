//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1197/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1197<F: Float>(t40380: F, t40397: F, t40400: F, t42172: F, t42173: F, t42176: F, t42180: F, t42184: F, t42188: F, t42190: F, t42194: F, t42198: F) -> F {
    let t48073 = F::new(0.51123901271894332903e0) * t40380;
    let t48074 = F::new(0.38342925953920749677e0) * t40397;
    let t48076 = F::new(0.76685851907841499354e0) * t40400;
    let t48078 = t48073 + t42172 + t42173 + t48074 - F::new(0.11502877786176224903e2) * t42176 - t48076 - F::new(0.92023022289409799224e1) * t42180 + t42184 - t42188 + t42190 - t42194 + t42198;
    t48078
}
