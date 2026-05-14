//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1088/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1088<F: Float>(t24019: F, t24025: F, t24037: F, t24044: F, t24076: F, t24137: F, t24141: F, t24202: F, t24206: F, t24215: F, t24223: F, t24225: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F) -> (F, F) {
    let t24664 = -t24019 + t24025 - t24037 + t24044 - t24076 - t24137 + t24141 + t24202 + t24206 - t24215 + t24223 + t24225;
    let t24677 = -0.42739999999999999999e0 * t23605 + 0.42739999999999999999e0 * t23670 - 0.35616666666666666666e-1 * t23608 - 0.47488888888888888888e-1 * t23673 - 0.11872222222222222222e0 * t23676 + 0.4274e0 * t23612 - 0.6411e0 * t23679 + 0.94977777777777777776e-1 * t23614 + 0.14246666666666666667e0 * t23616 - 0.14246666666666666667e0 * t23653 + 0.47488888888888888888e-1 * t23655;
    (t24664, t24677)
}
