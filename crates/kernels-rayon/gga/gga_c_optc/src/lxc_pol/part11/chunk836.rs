//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 836/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk836(t50: f64, t16236: f64, t16241: f64, t3373: f64, t4573: f64, t52: f64, t6724: f64, t16235: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t16245 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t6724 * t16236 + 4.0_f64 / 3.0_f64 * t3373 * t4573 + 4.0_f64 / 3.0_f64 * t52 * t16241);
    let t16247 = (t16235 + t16245) * t59;
    t16247
}
