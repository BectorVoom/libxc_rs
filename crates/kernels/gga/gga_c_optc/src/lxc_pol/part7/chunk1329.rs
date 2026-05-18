//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1329/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1329<F: Float>(t26314: F, t26319: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F, t26388: F) -> F {
    let t26523 = -F::new(0.89459259259259259259e0) * t26339 - F::new(0.301925e0) * t26343 - F::new(0.5519e0) * t26363 - F::new(0.18396666666666666667e0) * t26365 + F::new(0.22076e0) * t26367 + F::new(0.98115555555555555555e-1) * t26369 + F::new(0.40256666666666666668e0) * t26314 + F::new(0.11038e1) * t26372 - F::new(0.8585111111111111111e-1) * t26376 - F::new(0.82785e-1) * t26379 - F::new(0.22076e0) * t26382 + F::new(0.66228e0) * t26385 - F::new(0.11038e0) * t26388 + F::new(0.24154e1) * t26319;
    t26523
}
