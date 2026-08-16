//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 42/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk42(t107: f64, t27: f64, t29: f64, t38: f64) -> (f64, f64, f64, f64) {
    let t114 = 0.8e-1_f64 + 5.0_f64 / 18.0_f64 * t107 * t29 * t27 + 0.125e-1_f64 * t38;
    let t115 = t114 * t114;
    let t116 = t115 * t114;
    let t117 = 1.0_f64 / t116;
    (t114, t115, t116, t117)
}
