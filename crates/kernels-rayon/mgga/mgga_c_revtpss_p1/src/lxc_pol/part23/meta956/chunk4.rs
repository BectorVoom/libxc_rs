//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3195/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3195(t17605: f64, t21090: f64, t127: f64, t12988: f64, t24617: f64, t371: f64, t20842: f64, t5323: f64, t12784: f64, t12787: f64, t12866: f64, t17729: f64, t21182: f64, t24744: f64, t24804: f64, t44561: f64, t44797: f64, t5046: f64, t59062: f64, t6639: f64, t71278: f64, t71294: f64, t71297: f64) -> f64 {
    let t83916 = t17605 * t21090;
    let t83920 = t12988 * t371 * t127 * t24617;
    let t83922 = t5323 * t20842;
    let t83938 = 0.30488190661738479624e-2_f64 * t83916 - 0.85748036236139473947e-3_f64 * t83920 + 0.22866142996303859718e-2_f64 * t83922 + 0.7145669686344956162e-3_f64 * t12784 * t24804 - 0.71456696863449561621e-3_f64 * t17729 * t12787 * t5046 * t21182 + 0.45732285992607719436e-2_f64 * t71278 - t44797 + 0.28582678745379824648e-3_f64 * t71294 - t71297 / 144.0_f64 + 0.85748036236139473944e-3_f64 * t44561 * t24744 + 0.85748036236139473944e-3_f64 * t12866 * t59062 * t6639;
    t83938
}
