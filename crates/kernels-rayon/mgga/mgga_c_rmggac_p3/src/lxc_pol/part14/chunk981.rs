//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 981/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk981(t34760: f64, t9221: f64, t7457: f64, t16503: f64, t2281: f64, t34962: f64, t7467: f64, t14237: f64, t7482: f64, t2402: f64, t833: f64, t1587: f64, t2124: f64) -> (f64, f64, f64, f64, f64) {
    let t40771 = t9221 * t34760;
    let t40772 = t40771 * t7457;
    let t40776 = t16503 * t34962 * t2281 * t7467;
    let t40780 = t16503 * t14237 * t2281 * t7482;
    let t40785 = t2402 * t833;
    let t40788 = t2124 * t1587;
    (t40772, t40776, t40780, t40785, t40788)
}
