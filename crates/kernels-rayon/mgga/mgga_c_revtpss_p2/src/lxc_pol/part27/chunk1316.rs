//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1316/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1316(t26866: f64, t3746: f64, t12773: f64, t26867: f64, t12904: f64, t7618: f64, t12777: f64, t12781: f64, t12789: f64, t12842: f64, t12847: f64, t13002: f64, t13008: f64, t13022: f64, t13029: f64, t13048: f64, t13055: f64, t29097: f64, t29100: f64, t3606: f64, t3631: f64, t7607: f64, t97204: f64, t97206: f64, t97211: f64, t97215: f64, t97218: f64, t97220: f64, t97222: f64) -> f64 {
    let t97232 = t3746 * t26866;
    let t97239 = t26867 * t12773;
    let t97247 = t7618 * t12904;
    let t97249 = t97204 / 216.0_f64 + 0.25724410870841842183e-2_f64 * t97206 * t3606 + 0.25724410870841842183e-2_f64 * t97211 * t13048 - 0.25724410870841842183e-2_f64 * t97215 * t13055 + 0.17149607247227894789e-2_f64 * t97218 - t97220 / 288.0_f64 - t97222 / 144.0_f64 - t7607 * t13002 / 288.0_f64 - t7607 * t13008 / 48.0_f64 + t7607 * t13022 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t7607 * t13029 - 0.17149607247227894789e-2_f64 * t97232 * t3631 - 0.17149607247227894789e-2_f64 * t29097 * t12842 + 0.85748036236139473944e-3_f64 * t29100 * t12847 - 0.11433071498151929859e-2_f64 * t97239 - 0.85748036236139473944e-3_f64 * t26867 * t12777 - 0.17149607247227894789e-2_f64 * t26867 * t12781 + 0.14291339372689912324e-2_f64 * t26867 * t12789 - 0.28582678745379824648e-3_f64 * t97247;
    t97249
}
