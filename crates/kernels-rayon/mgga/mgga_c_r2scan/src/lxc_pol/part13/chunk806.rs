//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 806/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk806(t2266: f64, t6890: f64, t910: f64, t1543: f64, t288: f64, t97: f64, t2483: f64, t457: f64, t41: f64, t1524: f64, t963: f64, t6887: f64, t970: f64) -> (f64, f64, f64, f64, f64) {
    let t7116 = t2266 * t6890 * t910;
    let t7117 = 3.0_f64 * t7116;
    let t7118 = t1543 * t288;
    let t7120 = t97 * t7118 * t910;
    let t7121 = 6.0_f64 * t7120;
    let t7124 = t2483 * t457;
    let t7125 = t41 * t7124;
    let t7126 = 2.0_f64 * t7125;
    let t7127 = t963 * t1524;
    let t7128 = 0.11696447245269292414e1_f64 * t7127;
    let t7129 = t6887 * t970;
    (t7117, t7121, t7126, t7128, t7129)
}
