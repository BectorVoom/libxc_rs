//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3663/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3663<F: Float>(t12429: F, t12486: F, t12553: F, t16971: F, t17097: F, t17151: F, t20678: F, t20679: F, t3453: F, t3477: F, t3497: F, t3515: F, t3521: F, t45061: F, t45174: F, t5158: F, t6487: F, t6503: F, t6519: F, t6535: F, t68760: F, t68763: F, t68766: F, t68769: F, t68772: F, t68779: F, t68781: F, t68784: F, t68791: F, t68794: F) -> F {
    let t69216 = t68760 + t68763 + t68766 + t68769 - t68772 - t68779 - t68781 - t68784 + F::cast_from(0.20508037716432813316e4_f64) * t45174 * t20679 + F::cast_from(0.10254018858216406658e4_f64) * t12553 * t20678 * t3515 + F::cast_from(0.35089341735807877242e1_f64) * t3521 * t6535 * t3497 - t68791 - F::new(24.0) * t12429 * t6487 * t3453 + F::cast_from(0.70178683471615754484e1_f64) * t17097 * t16971 + F::new(6.0) * t3477 * t6503 * t3453 - F::cast_from(0.14035736694323150897e2_f64) * t12486 * t6519 * t3497 + t68794 + F::cast_from(0.11696447245269292414e1_f64) * t5158 * t17151 - F::cast_from(0.11696447245269292414e1_f64) * t45061 * t6519;
    t69216
}
