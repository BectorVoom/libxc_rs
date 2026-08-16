//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1046/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1046(t16503: f64, t16504: f64, t571: f64, t7467: f64, t3369: f64, t7482: f64, t34975: f64, t35039: f64, t38649: f64, t495: f64, t8440: f64, t275: f64, t8887: f64) -> (f64, f64, f64, f64) {
    let t41751 = t16503 * t16504 * t571 * t7467;
    let t41755 = t16503 * t3369 * t571 * t7482;
    let t41760 = t34975 * t35039 * t8440 * t38649 * t495;
    let t41763 = 2.0_f64 * t275 * t8887;
    (t41751, t41755, t41760, t41763)
}
