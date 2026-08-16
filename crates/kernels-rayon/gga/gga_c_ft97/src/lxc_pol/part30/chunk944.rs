//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 944/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk944(t141203: f64, t33485: f64, t375: f64, t89: f64, t33465: f64, t681: f64, t33469: f64, t33288: f64, t33308: f64, t7511: f64, t33303: f64, t33333: f64, t6109: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t141204 = 4.0_f64 / 9.0_f64 * t141203;
    let t141206 = t89 * t375 * t33485;
    let t141220 = t89 * t681 * t33465;
    let t141223 = t89 * t681 * t33469;
    let t141231 = t7511 * t33288 * t33308;
    let t141240 = t7511 * t33288 * t33303;
    let t141255 = t6109 * t681 * t33333;
    (t141204, t141206, t141220, t141223, t141231, t141240, t141255)
}
