//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 938/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk938(t3789: f64, t39: f64, t40: f64, t41547: f64, t13519: f64, t17836: f64, t24287: f64, t7453: f64, t15: f64, t33435: f64) -> (f64, f64, f64, f64) {
    let t140932 = t3789 * t41547 * t39 * t40;
    let t140937 = t17836 * t13519;
    let t140941 = 0.17024962234567901235e-1_f64 * t7453 * t24287;
    let t140943 = t33435 * t15;
    (t140932, t140937, t140941, t140943)
}
