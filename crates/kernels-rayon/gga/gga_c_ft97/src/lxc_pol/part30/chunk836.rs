//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 836/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk836(t35455: f64, t420: f64, t35454: f64, t236: f64, t5009: f64, t21251: f64, rho1: f64) -> (f64, f64, f64) {
    let t35456 = t420 * t35455;
    let t35457 = t35454 * t35456;
    let t35460 = t236 * t5009;
    let t35462 = 1.0_f64 / t21251 / rho1;
    (t35457, t35460, t35462)
}
