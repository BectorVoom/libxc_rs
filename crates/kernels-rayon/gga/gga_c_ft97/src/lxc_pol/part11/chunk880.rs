//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 880/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk880(t38052: f64, t38053: f64, t92: f64, t358: f64, t37391: f64, t378: f64, t11401: f64, t23: f64, t26: f64, t37357: f64, t37406: f64, t7954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38055 = t92 * t38052 * t38053;
    let t38057 = t358 * t37391;
    let t38059 = t92 * t378 * t38057;
    let t38061 = t11401 * t23;
    let t38062 = t26 * t38061;
    let t38063 = 280.0_f64 / 81.0_f64 * t38062;
    let t38064 = t37406 * t37357;
    let t38066 = t92 * t7954 * t38064;
    (t38055, t38057, t38059, t38061, t38062, t38063, t38064, t38066)
}
