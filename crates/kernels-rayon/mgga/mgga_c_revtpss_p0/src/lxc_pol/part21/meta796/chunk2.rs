//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2880/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880(t1610: f64, t41571: f64, t11289: f64, t4632: f64, t11510: f64, t1633: f64, t41224: f64, t981: f64, t15573: f64, t3022: f64, t11466: f64, t300: f64) -> (f64, f64, f64, f64, f64) {
    let t52229 = 1.0_f64 * t41571 * t1610;
    let t52231 = 3.0_f64 * t11289 * t4632;
    let t52235 = 0.12304822629859687989e5_f64 * t981 * t41224 * t1633 * t11510;
    let t52237 = 0.31168546390226634765e3_f64 * t3022 * t15573;
    let t52238 = t300 * t11466;
    (t52229, t52231, t52235, t52237, t52238)
}
