//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 479/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk479(t140: f64, t2036: f64, t543: f64, t5821: f64, t5824: f64, t7183: f64, t7191: f64, t7196: f64, t7207: f64, t7319: f64, t7335: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t7339 = piecewise3(t141, 0.10263553471742804997e0_f64 * t2036 * t7319 - 0.41054213886971219988e0_f64 * t543 * t7191 - 0.90629106640255751116e-1_f64 * t5821 * t7196 + 0.22653425206514361674e0_f64 * t543 * t7183 + 0.20527106943485609994e0_f64 * t140 * t7191 + 0.90629106640255751116e-1_f64 * t5824 * t7196 - 0.22653425206514361674e0_f64 * t140 * t7183 + 0.40013602467334010748e-1_f64 * t7335 * t7207, 0.0_f64);
    t7339
}
