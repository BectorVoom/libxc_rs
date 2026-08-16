//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2922/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922(t41329: f64, t41361: f64, t51978: f64, t52082: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64) -> f64 {
    let t77778 = 10.0_f64 / 81.0_f64 * t77499 - t77503 / 3.0_f64 + t77505 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t77507 + 2.0_f64 / 3.0_f64 * t77509 - 2.0_f64 / 3.0_f64 * t63276 + 2.0_f64 / 9.0_f64 * t63278 + t41329 + 4.0_f64 * t77515 - 10.0_f64 / 9.0_f64 * t77518 - 6.0_f64 * t77521 - t52082 + 28.0_f64 / 27.0_f64 * t51978 + 28.0_f64 / 81.0_f64 * t41361 - 2.0_f64 / 3.0_f64 * t77527 - 2.0_f64 / 3.0_f64 * t77531 + 8.0_f64 * t77535 - 6.0_f64 * t77539 + 2.0_f64 * t77543 + 2.0_f64 * t77547;
    t77778
}
