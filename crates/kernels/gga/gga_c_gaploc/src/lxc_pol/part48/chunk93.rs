//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 93/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk93<F: Float>(t397: F, t72: F, t109: F, t111: F, t112: F, t400: F, t427: F, t436: F, t437: F, t75: F) -> F {
    let t441 = t72 * t397;
    let t447 = F::new(0.13140859333333333333e-2) * t109 * t427 * t112 - F::new(0.98556444999999999995e-3) * t436 * t437 * t112 - F::new(0.19711288999999999999e-2) * t109 * t111 * t441 - F::new(4.0) * t75 * t400;
    t447
}
