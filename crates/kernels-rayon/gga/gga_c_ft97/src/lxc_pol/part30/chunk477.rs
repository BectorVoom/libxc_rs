//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 477/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk477(t292: f64, t1201: f64, t5265: f64, t7006: f64, t7009: f64, t7458: f64, t7466: f64, t7471: f64, t7480: f64, t7591: f64, t7607: f64) -> f64 {
    let t293 = 0.1e-59_f64 < t292;
    let t7611 = piecewise3(t293, 0.10263553471742804997e0_f64 * t5265 * t7591 - 0.41054213886971219988e0_f64 * t1201 * t7466 - 0.90629106640255751116e-1_f64 * t7006 * t7471 + 0.22653425206514361674e0_f64 * t1201 * t7458 + 0.20527106943485609994e0_f64 * t292 * t7466 + 0.90629106640255751116e-1_f64 * t7009 * t7471 - 0.22653425206514361674e0_f64 * t292 * t7458 + 0.40013602467334010748e-1_f64 * t7607 * t7480, 0.0_f64);
    t7611
}
