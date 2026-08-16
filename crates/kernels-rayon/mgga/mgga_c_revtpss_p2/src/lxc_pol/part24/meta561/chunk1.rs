//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1686/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686(t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64) -> f64 {
    let t88673 = 0.11111111111111111111e-1_f64 * t77505 - 0.24691358024691358025e-1_f64 * t88104 - 0.83333333333333333333e-2_f64 * t88108 - 0.44444444444444444444e-1_f64 * t77507 + 0.66666666666666666668e-1_f64 * t77509 + 0.11111111111111111111e0_f64 * t88114 - 0.55555555555555555555e-1_f64 * t88118 - 0.19999999999999999999e0_f64 * t88122 + 0.19999999999999999999e0_f64 * t88126 - 0.16666666666666666666e-1_f64 * t88130 - 0.22222222222222222222e-1_f64 * t88134 + 0.34567901234567901235e-1_f64 * t51978;
    t88673
}
