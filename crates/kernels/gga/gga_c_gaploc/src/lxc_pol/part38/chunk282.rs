//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 282/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk282<F: Float>(t2274: F, t2278: F, t2283: F, t2285: F, t471: F, t64: F, t869: F) -> F {
    let t2287 = -F::new(21.0) / F::new(256.0) * t2274 + F::new(21.0) / F::new(8192.0) * t2278 - F::new(7.0) / F::new(8192.0) * t2283 + F::new(7.0) / F::new(256.0) * t2285;
    let t2293 = t2287 * t471 - F::new(4.0) / F::new(3.0) * t869 * t64 - F::new(7.0) / F::new(256.0) * t2274 + F::new(7.0) / F::new(768.0) * t2285;
    t2293
}
