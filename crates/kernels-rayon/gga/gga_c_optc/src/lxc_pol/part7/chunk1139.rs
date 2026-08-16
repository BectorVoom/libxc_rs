//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1139/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1139(t23685: f64, t23651: f64, t23653: f64, t23655: f64, t23660: f64, t23664: f64, t23667: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64, t23769: f64) -> f64 {
    let t23770 = 0.12819753086419753086e4_f64 * t23685;
    let t23771 = 0.47199999999999999999e3_f64 * t23651 - 0.58153333333333333333e4_f64 * t23653 + 0.19384444444444444445e4_f64 * t23655 + 0.58153333333333333332e4_f64 * t23660 - 2832.0_f64 * t23664 + 0.62933333333333333332e3_f64 * t23667 + 17446.0_f64 * t23670 - 0.19384444444444444444e4_f64 * t23673 - 0.4846111111111111111e4_f64 * t23676 - 26169.0_f64 * t23679 + t23769 + t23770;
    t23771
}
