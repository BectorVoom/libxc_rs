//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 936/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk936(t22803: f64, t32247: f64, t3076: f64, t32167: f64, t7203: f64, t15: f64, t17: f64, t58: f64, t32239: f64, t32243: f64, t172: f64, t32151: f64) -> (f64, f64, f64, f64, f64) {
    let t136279 = t32247 * t22803;
    let t136282 = t3076 * t32167 * t7203;
    let t136299 = t58 * t15 * t17;
    let t136301 = t32239 * t136299 * t32243;
    let t136303 = t32151 * t172;
    (t136279, t136282, t136299, t136301, t136303)
}
