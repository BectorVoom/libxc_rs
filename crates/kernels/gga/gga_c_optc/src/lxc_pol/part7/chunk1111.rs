//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1111/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1111<F: Float>(t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F) -> F {
    let t23618 = -F::new(0.12315e-2) * t23581 - F::new(0.821e-2) * t23583 + F::new(0.3284e-2) * t23585 - F::new(0.19704e-1) * t23587 + F::new(0.14778e-1) * t23592 - F::new(0.1642e-2) * t23597 - F::new(0.3284e-2) * t23602 - F::new(0.46531999999999999999e2) * t23605 - F::new(0.38776666666666666665e1) * t23608 + F::new(0.46532e2) * t23612 + F::new(0.10340444444444444444e2) * t23614 + F::new(0.15510666666666666667e2) * t23616;
    t23618
}
