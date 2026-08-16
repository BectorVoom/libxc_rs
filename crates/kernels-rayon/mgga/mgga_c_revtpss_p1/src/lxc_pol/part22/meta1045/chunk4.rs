//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3663/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3663(t12429: f64, t12486: f64, t12553: f64, t16971: f64, t17097: f64, t17151: f64, t20678: f64, t20679: f64, t3453: f64, t3477: f64, t3497: f64, t3515: f64, t3521: f64, t45061: f64, t45174: f64, t5158: f64, t6487: f64, t6503: f64, t6519: f64, t6535: f64, t68760: f64, t68763: f64, t68766: f64, t68769: f64, t68772: f64, t68779: f64, t68781: f64, t68784: f64, t68791: f64, t68794: f64) -> f64 {
    let t69216 = t68760 + t68763 + t68766 + t68769 - t68772 - t68779 - t68781 - t68784 + 0.20508037716432813316e4_f64 * t45174 * t20679 + 0.10254018858216406658e4_f64 * t12553 * t20678 * t3515 + 0.35089341735807877242e1_f64 * t3521 * t6535 * t3497 - t68791 - 24.0_f64 * t12429 * t6487 * t3453 + 0.70178683471615754484e1_f64 * t17097 * t16971 + 6.0_f64 * t3477 * t6503 * t3453 - 0.14035736694323150897e2_f64 * t12486 * t6519 * t3497 + t68794 + 0.11696447245269292414e1_f64 * t5158 * t17151 - 0.11696447245269292414e1_f64 * t45061 * t6519;
    t69216
}
