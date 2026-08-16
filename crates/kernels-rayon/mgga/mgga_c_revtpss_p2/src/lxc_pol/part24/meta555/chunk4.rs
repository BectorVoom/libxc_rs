//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1660/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660(t141: f64, t2908: f64, t88128: f64, t41246: f64, t77499: f64, t77505: f64, t77507: f64, t77509: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64, t88144: f64, t88147: f64, t88150: f64, t88161: f64) -> (f64, f64) {
    let t88164 = t141 * t2908 * t88128;
    let t88166 = -0.8585111111111111111e-1_f64 * t88144 - 0.82785e-1_f64 * t88147 + 0.44152e0_f64 * t88150 - 0.44152e0_f64 * t77663 + 0.98115555555555555555e-1_f64 * t77667 - 0.108693e2_f64 * t88089 + 0.24154e1_f64 * t88097 + t41246 + 0.44729629629629629629e0_f64 * t77499 + 0.40256666666666666668e0_f64 * t77505 - 0.16102666666666666667e1_f64 * t77507 + 0.24154e1_f64 * t77509 - 0.99342e0_f64 * t88161 - 0.82785e-1_f64 * t88164;
    (t88164, t88166)
}
