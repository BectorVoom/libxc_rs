//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2218/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2218<F: Float>(t12623: F, t12656: F, t12662: F, t12665: F, t1411: F, t1426: F, t2251: F, t2304: F, t3962: F, t3968: F, t3971: F, t3997: F, t607: F, t642: F, t67: F, t80: F, t9248: F, t9259: F, t9339: F) -> F {
    let t46050 = -t12662 * t642 / F::cast_from(4.0_f64) - t12623 * t642 / F::cast_from(4.0_f64) - t12656 * t642 / F::cast_from(2.0_f64) - t12665 * t642 / F::cast_from(2.0_f64) - t3971 * t2304 / F::cast_from(4.0_f64) - t3962 * t2304 / F::cast_from(4.0_f64) - t3968 * t2304 / F::cast_from(4.0_f64) - t1411 * t9339 / F::cast_from(12.0_f64) - t607 * t1426 * t67 * t9248 / F::cast_from(4.0_f64) - t9259 * t1426 * t80 / F::cast_from(12.0_f64) - t2251 * t3997 * t80 / F::cast_from(4.0_f64);
    t46050
}
