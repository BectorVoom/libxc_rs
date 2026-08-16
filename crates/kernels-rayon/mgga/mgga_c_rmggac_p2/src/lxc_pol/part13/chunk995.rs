//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 995/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk995(t16156: f64, t9198: f64, t388: f64, t575: f64, t7933: f64, t7934: f64, t535: f64, t7244: f64, t8422: f64, t1598: f64, t16503: f64, t16504: f64, t7448: f64) -> (f64, f64, f64, f64, f64) {
    let t41813 = t16156 * t9198;
    let t41817 = t7933 * t7934 * t388 * t575;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41828 = t7244 * t8422;
    let t41834 = t16503 * t16504 * t1598 * t7448;
    (t41813, t41817, t41821, t41828, t41834)
}
