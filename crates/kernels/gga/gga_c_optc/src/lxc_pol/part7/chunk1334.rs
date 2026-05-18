//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1334/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1334<F: Float>(t26314: F, t26319: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F, t26388: F) -> F {
    let t26626 = -F::new(0.15302962962962962963e1) * t26339 - F::new(0.516475e0) * t26343 - F::new(0.69463333333333333334e0) * t26363 - F::new(0.23154444444444444445e0) * t26365 + F::new(0.27785333333333333333e0) * t26367 + F::new(0.12349037037037037037e0) * t26369 + F::new(0.68863333333333333332e0) * t26314 + F::new(0.13892666666666666667e1) * t26372 - F::new(0.10805407407407407407e0) * t26376 - F::new(0.104195e0) * t26379 - F::new(0.27785333333333333334e0) * t26382 + F::new(0.83356e0) * t26385 - F::new(0.13892666666666666667e0) * t26388 + F::new(0.41318e1) * t26319;
    t26626
}
