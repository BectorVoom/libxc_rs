//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1381/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381(t10782: f64, t40731: f64, t159: f64, t33127: f64, t64: f64, t222: f64, t10777: f64, t10779: f64, t2749: f64, t40578: f64, t10627: f64, t10900: f64, t125: f64, t2430: f64, t2731: f64, t2745: f64, t39476: f64, t40673: f64, t40679: f64, t40681: f64, t40686: f64, t40691: f64, t40696: f64, t40700: f64, t40705: f64, t40707: f64, t40711: f64, t40719: f64, t40722: f64, t40728: f64, t800: f64, t828: f64, t837: f64, t851: f64, t855: f64) -> (f64, f64) {
    let t40732 = t40731 * t10782;
    let t40735 = t64 * t33127 * t159;
    let t40737 = 455.0_f64 / 243.0_f64 * t40735 * t222;
    let t40744 = t10777 * t10779 * t40578 * t2749;
    let t40746 = 0.10289764348336736874e0_f64 * t2745 * t40673 * t125 * t10627 * t837 + 0.91464571985215438873e-3_f64 * t40679 - 0.16262400898971305032e-1_f64 * t40681 - 0.6098400337114239387e-2_f64 * t40686 + 0.45178982497454656791e-6_f64 * t40691 + 0.17149607247227894789e-3_f64 * t40696 - 0.17149607247227894789e-3_f64 * t40700 + 0.28582678745379824648e-4_f64 * t40705 - 0.13605355082800796532e0_f64 * t40707 - 0.20553867802866510526e-1_f64 * t40711 - 0.85748036236139473944e-3_f64 * t851 * t855 * t828 * t39476 - 0.2032800112371413129e-3_f64 * t40719 - 0.73180804045370872643e-3_f64 * t40722 - 0.30492001685571196936e-2_f64 * t40728 - 0.65049603595885220128e-2_f64 * t40732 + t40737 - 3.0_f64 / 2.0_f64 * t10900 * t800 * t2731 * t2430 + 0.60984003371142393869e-3_f64 * t40744;
    (t40735, t40746)
}
