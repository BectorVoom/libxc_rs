//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 83/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk83(t194: f64, t272: f64, t322: f64, t170: f64, t173: f64) -> (f64, f64, f64, f64, f64) {
    let t325 = 0.469508e0_f64 * t272 + 0.4332925e0_f64 * t194;
    let t326 = t325 * t325;
    let t327 = 1.0_f64 / t326;
    let t328 = t322 * t327;
    let t332 = f64::exp(-t170 * t173 * t328 / 4.0_f64);
    (t325, t326, t327, t328, t332)
}
