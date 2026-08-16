//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 932/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk932<F: Float>(t1411: F, t1427: F, t1434: F, t19322: F, t20207: F, t20210: F, t20219: F, t20222: F, t20227: F, t20265: F, t20285: F, t5393: F, t5400: F, t5403: F, t5428: F, t5442: F, t66: F, t80: F) -> F {
    let t20288 = -t19322 * t20207 / F::cast_from(4.0_f64) - t20210 * t80 / F::cast_from(4.0_f64) - t5393 * t1434 / F::cast_from(4.0_f64) - t20219 * t80 / F::cast_from(12.0_f64) - t20222 * t80 / F::cast_from(4.0_f64) - t5400 * t1434 / F::cast_from(4.0_f64) - t20227 * t80 / F::cast_from(4.0_f64) - t5403 * t1434 / F::cast_from(2.0_f64) - t1411 * t5442 / F::cast_from(4.0_f64) + t20265 * t80 / F::cast_from(24.0_f64) + t5428 * t1434 / F::cast_from(8.0_f64) + t1427 * t5442 / F::cast_from(8.0_f64) + t66 * t20285 / F::cast_from(24.0_f64);
    t20288
}
