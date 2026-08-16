//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1662/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662(t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64) -> f64 {
    let t88201 = 4.0_f64 / 9.0_f64 * t77505 - 80.0_f64 / 81.0_f64 * t88104 - t88108 / 3.0_f64 - 16.0_f64 / 9.0_f64 * t77507 + 8.0_f64 / 3.0_f64 * t77509 + 40.0_f64 / 9.0_f64 * t88114 - 20.0_f64 / 9.0_f64 * t88118 - 8.0_f64 * t88122 + 8.0_f64 * t88126 - 2.0_f64 / 3.0_f64 * t88130 - 8.0_f64 / 9.0_f64 * t88134 + 112.0_f64 / 81.0_f64 * t51978;
    t88201
}
