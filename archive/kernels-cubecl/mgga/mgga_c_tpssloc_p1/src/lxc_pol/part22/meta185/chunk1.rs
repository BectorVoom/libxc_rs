//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1103/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1103<F: Float>(t1411: F, t1427: F, t1434: F, t5393: F, t5400: F, t5403: F, t5428: F, t5442: F, t66: F, t80: F) -> F {
    let t5445 = -t5393 * t80 / F::cast_from(12.0_f64) - t5400 * t80 / F::cast_from(12.0_f64) - t5403 * t80 / F::cast_from(6.0_f64) - t1411 * t1434 / F::cast_from(6.0_f64) + t5428 * t80 / F::cast_from(24.0_f64) + t1427 * t1434 / F::cast_from(12.0_f64) + t66 * t5442 / F::cast_from(24.0_f64);
    t5445
}
