//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 819/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk819(t1965: f64, t1967: f64, t28: f64, t8511: f64, t118: f64, t1986: f64, t352: f64, t39866: f64, t2318: f64, t326: f64, t333: f64, t551: f64, t7817: f64) -> (f64, f64, f64, f64) {
    let t40278 = t8511 * t1965 * t1967 * t28;
    let t40313 = t1986 * t118 * t39866 * t352;
    let t40323 = t1986 * t326 * t2318 * t333;
    let t40331 = t7817 * t551;
    (t40278, t40313, t40323, t40331)
}
