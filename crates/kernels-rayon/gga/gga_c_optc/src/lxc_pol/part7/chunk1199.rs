//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1199/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1199(t2512: f64, t2485: f64, t2492: f64, t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64, t23653: f64, t23655: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64) -> (f64, f64, f64) {
    let t24752 = t2512 * t2512;
    let t24759 = t2485 * t2492;
    let t24775 = -0.41095999999999999999e0_f64 * t23605 + 0.41095999999999999998e0_f64 * t23670 - 0.34246666666666666665e-1_f64 * t23608 - 0.4566222222222222222e-1_f64 * t23673 - 0.11415555555555555555e0_f64 * t23676 + 0.41096e0_f64 * t23612 - 0.61644e0_f64 * t23679 + 0.9132444444444444444e-1_f64 * t23614 + 0.13698666666666666667e0_f64 * t23616 - 0.13698666666666666667e0_f64 * t23653 + 0.4566222222222222222e-1_f64 * t23655;
    (t24752, t24759, t24775)
}
