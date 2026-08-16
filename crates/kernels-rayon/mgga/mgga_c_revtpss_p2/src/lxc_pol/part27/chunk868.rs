//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 868/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk868(t10430: f64, t10432: f64, t10435: f64, t10438: f64, t10442: f64, t10444: f64, t10469: f64, t10489: f64, t198: f64, t765: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> f64 {
    let t10493 = 3.0_f64 * t10489 * t198 * t765 + t10430 + t10432 + t10435 + t10438 + t10442 + t10444 + t10469 - t9278 + t9308 + t9316 + t9329 + t9333;
    t10493
}
