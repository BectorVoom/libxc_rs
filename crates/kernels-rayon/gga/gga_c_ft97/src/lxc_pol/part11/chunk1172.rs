//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1172/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1172(t299: f64, t43297: f64, t44795: f64, t10188: f64, t10944: f64, t13: f64, t39375: f64, t41401: f64, t43034: f64, t8613: f64, t9479: f64) -> f64 {
    let t300 = 10000000.0_f64 <= t299;
    let t44797 = piecewise3(t300, 0.0_f64, t43297 + t44795);
    let tv4rho40 = 4.0_f64 * t8613 + 4.0_f64 * t9479 + 4.0_f64 * t10188 + 4.0_f64 * t10944 + t13 * (t39375 + t41401 + t43034 + t44797);
    tv4rho40
}
