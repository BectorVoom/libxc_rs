//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1350/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1350(t62390: f64, t67160: f64, t67162: f64, t67169: f64, t69531: f64, t69533: f64, t69535: f64, t69537: f64, t69539: f64, t69541: f64, t69544: f64, t69546: f64, t69548: f64) -> f64 {
    let t71798 = -t67160 - t67162 + 7.0_f64 / 1152.0_f64 * t69531 - 7.0_f64 / 576.0_f64 * t69533 + 7.0_f64 / 1152.0_f64 * t69535 - 5.0_f64 / 32.0_f64 * t69537 + 5.0_f64 / 96.0_f64 * t69539 + 5.0_f64 / 192.0_f64 * t69541 + t69544 / 8.0_f64 - t69546 / 24.0_f64 - t67169 + t69548 / 192.0_f64 - t62390;
    t71798
}
