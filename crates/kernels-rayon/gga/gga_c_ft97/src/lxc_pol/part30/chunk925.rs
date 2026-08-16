//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 925/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk925(t123124: f64, t27565: f64, t109200: f64, t17836: f64, t6: f64, t24389: f64, t39: f64, t108585: f64, t17817: f64, t27657: f64, t3766: f64, t22511: f64, t33432: f64, t3789: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123125 = t123124 * t27565;
    let t123129 = t17836 * t109200 * t6;
    let t123133 = t17836 * t24389 * t39;
    let t123181 = t17817 * t108585;
    let t123408 = t3766 * t27657;
    let t123445 = t3789 * t33432 * t22511;
    (t123125, t123129, t123133, t123181, t123408, t123445)
}
