//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1306/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1306(t11283: f64, t11297: f64, t11611: f64, t33091: f64, t33093: f64, t33094: f64, t33095: f64, t33096: f64, t33097: f64, t33098: f64, t33099: f64, t33100: f64, t33101: f64, t36085: f64, t36086: f64, t7: f64) -> f64 {
    let t36089 = 4.0_f64 * t11283;
    let t36090 = 2.0_f64 * t11297;
    let tv4rho2sigma20 = t33091 + 2.0_f64 * t11611 + t33093 - t33094 + t33095 - t33096 + t33097 + t33098 - t33099 - t33100 + t33101 + t7 * (t36085 + t36086) - t36089 - t36090;
    tv4rho2sigma20
}
