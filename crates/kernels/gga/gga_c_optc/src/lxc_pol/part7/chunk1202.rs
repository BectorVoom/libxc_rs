//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1202/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1202<F: Float>(t23576: F, t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23620: F) -> F {
    let t24839 = -F::cast_from(0.10805407407407407407e0_f64) * t23576 - F::cast_from(0.104195e0_f64) * t23581 - F::cast_from(0.69463333333333333334e0_f64) * t23583 + F::cast_from(0.27785333333333333334e0_f64) * t23585 - F::cast_from(0.166712e1_f64) * t23587 + F::cast_from(0.125034e1_f64) * t23592 - F::cast_from(0.13892666666666666667e0_f64) * t23597 - F::cast_from(0.27785333333333333334e0_f64) * t23602 - F::cast_from(0.123954e2_f64) * t23605 - F::cast_from(0.103295e1_f64) * t23608 + F::cast_from(0.123954e2_f64) * t23612 + F::cast_from(0.27545333333333333333e1_f64) * t23614 + F::cast_from(0.41318e1_f64) * t23616 - F::cast_from(0.13772666666666666666e1_f64) * t23620;
    t24839
}
