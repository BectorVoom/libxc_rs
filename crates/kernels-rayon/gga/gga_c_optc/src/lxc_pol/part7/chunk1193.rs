//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1193/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1193(t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64, t23653: f64, t23655: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64, t23682: f64) -> (f64, f64) {
    let t24677 = -0.42739999999999999999e0_f64 * t23605 + 0.42739999999999999999e0_f64 * t23670 - 0.35616666666666666666e-1_f64 * t23608 - 0.47488888888888888888e-1_f64 * t23673 - 0.11872222222222222222e0_f64 * t23676 + 0.4274e0_f64 * t23612 - 0.6411e0_f64 * t23679 + 0.94977777777777777776e-1_f64 * t23614 + 0.14246666666666666667e0_f64 * t23616 - 0.14246666666666666667e0_f64 * t23653 + 0.47488888888888888888e-1_f64 * t23655;
    let t24678 = 0.18467901234567901234e0_f64 * t23682;
    (t24677, t24678)
}
