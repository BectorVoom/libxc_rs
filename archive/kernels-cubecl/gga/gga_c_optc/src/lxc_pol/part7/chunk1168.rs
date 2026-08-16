//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1168/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1168<F: Float>(t23576: F, t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23620: F) -> F {
    let t24263 = -F::cast_from(0.85199506172839506175e-1_f64) * t23576 - F::cast_from(0.82156666666666666667e-1_f64) * t23581 - F::cast_from(0.54771111111111111111e0_f64) * t23583 + F::cast_from(0.21908444444444444444e0_f64) * t23585 - F::cast_from(0.13145066666666666666e1_f64) * t23587 + F::cast_from(0.98587999999999999999e0_f64) * t23592 - F::cast_from(0.10954222222222222222e0_f64) * t23597 - F::cast_from(0.21908444444444444444e0_f64) * t23602 - F::cast_from(0.71752000000000000002e1_f64) * t23605 - F::cast_from(0.59793333333333333333e0_f64) * t23608 + F::cast_from(0.71752e1_f64) * t23612 + F::cast_from(0.15944888888888888889e1_f64) * t23614 + F::cast_from(0.23917333333333333333e1_f64) * t23616 - F::cast_from(0.79724444444444444446e0_f64) * t23620;
    t24263
}
