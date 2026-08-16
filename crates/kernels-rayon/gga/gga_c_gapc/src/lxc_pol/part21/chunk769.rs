//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 769/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk769(t9009: f64, t9011: f64, t9014: f64, t9017: f64, t9021: f64, t9024: f64, t9027: f64, t9032: f64, t9034: f64, t9036: f64, t9038: f64, t9042: f64, t9044: f64) -> f64 {
    let t9046 = -0.12357942809624928455e-3_f64 * t9009 - 0.18326250058315256483e-6_f64 * t9011 - 0.27801896084645508334e-2_f64 * t9014 + 0.75883739738679928911e-6_f64 * t9017 - 0.13492128925537291361e-5_f64 * t9021 - 0.7588373973867992891e-7_f64 * t9024 + 0.13492128925537291361e-6_f64 * t9027 + 0.7324140771837707598e-5_f64 * t9032 - 0.2318836277704281739e-4_f64 * t9034 + 0.56360603971979070047e-7_f64 * t9036 - 0.10020915386217878654e-6_f64 * t9038 + 0.56275309320814680968e-8_f64 * t9042 + 0.5627530932081468097e-7_f64 * t9044;
    t9046
}
