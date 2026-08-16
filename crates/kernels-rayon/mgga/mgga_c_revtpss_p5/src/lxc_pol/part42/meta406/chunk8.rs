//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1420/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1420(t21876: f64, t655: f64, t10201: f64, t10202: f64, t13448: f64, t13451: f64, t13453: f64, t21818: f64, t21821: f64, t21824: f64, t21827: f64, t21830: f64, t69: f64) -> f64 {
    let t21877 = t655 * t21876;
    let t21880 = -t10201 - 11.0_f64 / 9.0_f64 * t10202 - 22.0_f64 / 9.0_f64 * t13448 - t13451 + t13453 - 2.0_f64 / 3.0_f64 * t21818 - 3.0_f64 / 4.0_f64 * t69 * t21821 + t69 * t21824 / 2.0_f64 + t21827 / 3.0_f64 + t69 * t21830 / 4.0_f64 - t69 * t21877 / 8.0_f64;
    t21880
}
