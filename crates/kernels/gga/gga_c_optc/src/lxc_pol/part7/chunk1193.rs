//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1193/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1193<F: Float>(t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F, t23682: F) -> (F, F) {
    let t24677 = -F::new(0.42739999999999999999e0) * t23605 + F::new(0.42739999999999999999e0) * t23670 - F::new(0.35616666666666666666e-1) * t23608 - F::new(0.47488888888888888888e-1) * t23673 - F::new(0.11872222222222222222e0) * t23676 + F::new(0.4274e0) * t23612 - F::new(0.6411e0) * t23679 + F::new(0.94977777777777777776e-1) * t23614 + F::new(0.14246666666666666667e0) * t23616 - F::new(0.14246666666666666667e0) * t23653 + F::new(0.47488888888888888888e-1) * t23655;
    let t24678 = F::new(0.18467901234567901234e0) * t23682;
    (t24677, t24678)
}
