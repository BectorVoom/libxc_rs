//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3133/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133(t13036: f64, t3597: f64, t57403: f64, t12772: f64, t17678: f64, t5340: f64, t17683: f64, t5331: f64, t1222: f64, t12809: f64, t12876: f64, t12910: f64, t13048: f64, t13055: f64, t16771: f64, t17461: f64, t21306: f64, t3720: f64, t44624: f64, t44649: f64, t44658: f64, t44661: f64, t44672: f64, t5308: f64, t5332: f64, t5405: f64, t56153: f64, t56224: f64, t57735: f64, t57737: f64, t57743: f64, t57746: f64, t57749: f64, t57759: f64) -> f64 {
    let t57763 = t13036 * t3597 * t57403;
    let t57770 = t5340 * t12772 * t17678;
    let t57773 = t5331 * t12772 * t17683;
    let t57779 = -0.64311027177104605458e-3_f64 * t21306 * t12876 + 0.25724410870841842183e-2_f64 * t44624 * t17461 - 0.17149607247227894789e-2_f64 * t57735 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t5332 * t57737 - t57743 / 72.0_f64 - t57746 / 144.0_f64 - t57749 / 48.0_f64 - t1222 * t5308 * t56224 / 48.0_f64 - t1222 * t5308 * t56153 / 48.0_f64 + 0.28582678745379824648e-3_f64 * t44649 - 0.68598428988911579154e-2_f64 * t57759 * t13048 + 0.68598428988911579154e-2_f64 * t57763 * t13055 + 0.85748036236139473944e-3_f64 * t44658 + 0.85748036236139473944e-3_f64 * t44661 - 0.42874018118069736972e-3_f64 * t44672 - 0.57165357490759649295e-3_f64 * t57770 + 0.28582678745379824648e-3_f64 * t57773 + 0.25724410870841842183e-2_f64 * t12910 * t3720 * t16771 * t5405;
    t57779
}
