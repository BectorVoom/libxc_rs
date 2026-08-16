//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1273/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1273(t28190: f64, t28214: f64, t100814: f64, t100817: f64, t100820: f64, t100823: f64, t26966: f64, t29104: f64, t8091: f64, t96412: f64, t97010: f64, t97442: f64, t97449: f64, t97465: f64) -> f64 {
    let t100830 = t28190 * t28214;
    let t100832 = 0.10317654320987654321e-2_f64 * t100814 + 0.92858888888888888885e-2_f64 * t100817 + 0.61905925925925925925e-2_f64 * t100820 - 0.41270617283950617283e-2_f64 * t100823 - 0.61782407407407407408e-3_f64 * t26966 * t29104 + t97442 - t97449 - 0.51588271604938271603e-3_f64 * t96412 + t97465 + 0.61782407407407407408e-3_f64 * t97010 * t8091 - 0.7722800925925925926e-4_f64 * t100830;
    t100832
}
