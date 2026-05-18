//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1199/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1199<F: Float>(t2512: F, t2485: F, t2492: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F) -> (F, F, F) {
    let t24752 = t2512 * t2512;
    let t24759 = t2485 * t2492;
    let t24775 = -F::new(0.41095999999999999999e0) * t23605 + F::new(0.41095999999999999998e0) * t23670 - F::new(0.34246666666666666665e-1) * t23608 - F::new(0.4566222222222222222e-1) * t23673 - F::new(0.11415555555555555555e0) * t23676 + F::new(0.41096e0) * t23612 - F::new(0.61644e0) * t23679 + F::new(0.9132444444444444444e-1) * t23614 + F::new(0.13698666666666666667e0) * t23616 - F::new(0.13698666666666666667e0) * t23653 + F::new(0.4566222222222222222e-1) * t23655;
    (t24752, t24759, t24775)
}
