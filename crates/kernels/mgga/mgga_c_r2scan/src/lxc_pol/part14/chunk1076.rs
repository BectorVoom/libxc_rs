//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1076/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1076<F: Float>(t37835: F, t37838: F, t37841: F, t37843: F, t37848: F, t37851: F, t38528: F, t38532: F, t39740: F, t39742: F, t39746: F, t39749: F, t39762: F, t37859: F, t37881: F, t39759: F, t39765: F, t39768: F, t39770: F, t39772: F, t39775: F, t39778: F, t39780: F, t39782: F) -> (F, F) {
    let t41537 = t38528 + t38532 + 0.11708928647259339623e0 * t37835 + 0.90044238659382329742e0 * t37838 + 0.27013271597814698923e1 * t37841 - 0.17336443480108537126e0 * t39740 - 0.86682217400542685632e-1 * t39742 - 0.5200933044032561138e0 * t39746 + 0.26198215989259945076e-1 * t39749 + 0.54878743191129263322e-2 * t37843 - 0.16951189180550569635e1 * t37848 - 0.50853567541651708904e1 * t37851;
    let t41542 = 0.13869154784086829701e1 * t39762;
    let t41551 = 0.46230515946956099004e0 * t37859 + 0.95219938395347901946e-2 * t37881 - 0.10401866088065122276e1 * t39759 - t41542 - 0.5200933044032561138e0 * t39765 - 0.52009330440325611378e0 * t39768 + 0.51220160311720645766e0 * t39770 - 0.85366933852867742943e0 * t39772 - 0.17465477326173296718e-1 * t39775 - 0.26198215989259945076e-1 * t39778 - 0.26198215989259945076e-1 * t39780 - 0.1047928639570397803e0 * t39782;
    (t41537, t41551)
}
