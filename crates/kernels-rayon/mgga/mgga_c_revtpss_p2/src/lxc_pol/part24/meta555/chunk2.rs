//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1658/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1658(t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64) -> f64 {
    let t88137 = 0.23744444444444444444e-1_f64 * t77505 - 0.52765432098765432099e-1_f64 * t88104 - 0.17808333333333333333e-1_f64 * t88108 - 0.94977777777777777776e-1_f64 * t77507 + 0.14246666666666666667e0_f64 * t77509 + 0.23744444444444444444e0_f64 * t88114 - 0.11872222222222222222e0_f64 * t88118 - 0.42739999999999999999e0_f64 * t88122 + 0.42739999999999999999e0_f64 * t88126 - 0.35616666666666666666e-1_f64 * t88130 - 0.47488888888888888888e-1_f64 * t88134 + 0.73871604938271604937e-1_f64 * t51978;
    t88137
}
