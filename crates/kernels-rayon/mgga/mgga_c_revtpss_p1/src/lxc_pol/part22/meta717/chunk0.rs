//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2752/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2752(t10073: f64, t10934: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64, t22: f64, t251: f64, t837: f64, t10111: f64, t2789: f64, t588: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39694 = t10073 * t10934;
    let t39697 = 0.88356352675825229576e-3_f64 * t39552 * t253;
    let t39698 = t9646 * t2783;
    let t39701 = t39698 * t251 * t22 * t837;
    let t39719 = t10111 * t2789 * t22;
    let t39723 = 0.15709759505761725819e-2_f64 * t10111 * t870 * t588;
    (t39694, t39697, t39698, t39701, t39719, t39723)
}
