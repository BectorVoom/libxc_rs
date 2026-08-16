//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 845/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk845(t9595: f64, t9597: f64, t9600: f64, t9603: f64, t9607: f64, t9610: f64, t9614: f64, t9616: f64, t9618: f64, t9621: f64, t9625: f64, t9628: f64, t9631: f64) -> f64 {
    let t9633 = 0.28985453471303521736e-5_f64 * t9595 - 0.28985453471303521736e-5_f64 * t9597 + 0.43478180206955282604e-5_f64 * t9600 - 0.61900849231692170544e-6_f64 * t9603 + 0.50680539737635041234e-4_f64 * t9607 - 0.17376185052903442709e-3_f64 * t9610 - 0.17376185052903442709e-3_f64 * t9614 - 0.12163329537032409896e-2_f64 * t9616 + 0.42270452978984302532e-6_f64 * t9618 - 0.13900948042322754167e-2_f64 * t9621 + 0.10120442708333333334e-4_f64 * t9625 + 0.50602213541666666668e-4_f64 * t9628 + 0.50602213541666666668e-4_f64 * t9631;
    t9633
}
