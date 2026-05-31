//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 452/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk452<F: Float>(t2274: F, t2278: F, t2283: F, t2285: F, t471: F, t64: F, t869: F) -> (F, F) {
    let t2287 = -F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t2274 + F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t2278 - F::cast_from(7.0_f64) / F::cast_from(8192.0_f64) * t2283 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t2285;
    let t2293 = t2287 * t471 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t869 * t64 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t2274 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t2285;
    (t2287, t2293)
}
