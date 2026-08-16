//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 861/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk861(t236: f64, t495: f64, t7230: f64, t9210: f64, t9211: f64, t2145: f64, t27: f64, t5249: f64, t649: f64, t34847: f64, t9118: f64, t16156: f64, t9111: f64) -> (f64, f64, f64, f64) {
    let t39224 = t7230 * t9210 * t236 * t9211 * t495;
    let t39228 = t2145 * t27 * t649 * t5249;
    let t39231 = t34847 * t9118;
    let t39233 = t16156 * t9111;
    (t39224, t39228, t39231, t39233)
}
