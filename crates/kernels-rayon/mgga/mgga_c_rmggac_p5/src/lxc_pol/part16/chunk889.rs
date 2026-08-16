//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 889/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk889(t262: f64, t44736: f64, t7204: f64, t34884: f64, t9971: f64, t16503: f64, t35039: f64, t571: f64, t8420: f64, t16504: f64, t8425: f64, t1598: f64, t9163: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44737 = t262 * t44736;
    let t44738 = t7204 * t44737;
    let t44740 = t34884 * t9971;
    let t44744 = t16503 * t35039 * t571 * t8420;
    let t44748 = t16503 * t16504 * t571 * t8425;
    let t44752 = t16503 * t16504 * t1598 * t9163;
    (t44737, t44738, t44740, t44744, t44748, t44752)
}
