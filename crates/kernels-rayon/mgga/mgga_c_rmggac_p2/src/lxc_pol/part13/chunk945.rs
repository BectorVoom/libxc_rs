//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 945/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk945(t40756: f64, t903: f64, t1679: f64, t7197: f64, t7200: f64, t38530: f64, t7484: f64, t7450: f64, t34760: f64, t9221: f64, t7457: f64, t16503: f64, t2281: f64, t34962: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40757 = t903 * t40756;
    let t40759 = t1679 * t7197;
    let t40760 = t40759 * t7200;
    let t40762 = t38530 * t7484;
    let t40764 = t38530 * t7450;
    let t40771 = t9221 * t34760;
    let t40772 = t40771 * t7457;
    let t40776 = t16503 * t34962 * t2281 * t7467;
    (t40757, t40760, t40762, t40764, t40772, t40776)
}
