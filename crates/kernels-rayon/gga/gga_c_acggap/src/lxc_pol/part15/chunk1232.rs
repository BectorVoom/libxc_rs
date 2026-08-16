//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1232/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1232(t30793: f64, t35004: f64, t35012: f64, t35018: f64, t35022: f64, t35024: f64, t37321: f64, t39581: f64, t39585: f64, t39587: f64, t39592: f64, t39594: f64, t39599: f64, t39601: f64, t39605: f64, t39607: f64, t39609: f64) -> f64 {
    let t41721 = 0.21437009059034868486e-2_f64 * t39581 + 0.14291339372689912324e-2_f64 * t39585 - 0.68598428988911579156e-2_f64 * t39587 - 0.12579236915841660828e-2_f64 * t39592 + t37321 + 0.68598428988911579156e-2_f64 * t39594 + 0.42874018118069736972e-3_f64 * t39599 + t39601 / 8.0_f64 - 0.37737710747524982482e-2_f64 * t30793 - t35004 + 0.68598428988911579156e-2_f64 * t39605 + 0.68598428988911579156e-2_f64 * t39607 - 0.37737710747524982482e-1_f64 * t39609 - t35012 + t35018 + 0.11433071498151929859e-2_f64 * t35022 - t35024;
    t41721
}
