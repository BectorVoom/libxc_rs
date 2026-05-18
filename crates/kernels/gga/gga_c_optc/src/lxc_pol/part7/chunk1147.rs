//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1147/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1147<F: Float>(t23576: F, t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23620: F) -> F {
    let t23897 = -F::new(0.8585111111111111111e-1) * t23576 - F::new(0.82785e-1) * t23581 - F::new(0.5519e0) * t23583 + F::new(0.22076e0) * t23585 - F::new(0.132456e1) * t23587 + F::new(0.99342e0) * t23592 - F::new(0.11038e0) * t23597 - F::new(0.22076e0) * t23602 - F::new(0.72462e1) * t23605 - F::new(0.60384999999999999999e0) * t23608 + F::new(0.72462e1) * t23612 + F::new(0.16102666666666666667e1) * t23614 + F::new(0.24154e1) * t23616 - F::new(0.80513333333333333336e0) * t23620;
    t23897
}
