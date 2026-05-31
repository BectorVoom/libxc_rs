//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1381/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381<F: Float>(t10782: F, t40731: F, t159: F, t33127: F, t64: F, t222: F, t10777: F, t10779: F, t2749: F, t40578: F, t10627: F, t10900: F, t125: F, t2430: F, t2731: F, t2745: F, t39476: F, t40673: F, t40679: F, t40681: F, t40686: F, t40691: F, t40696: F, t40700: F, t40705: F, t40707: F, t40711: F, t40719: F, t40722: F, t40728: F, t800: F, t828: F, t837: F, t851: F, t855: F) -> (F, F) {
    let t40732 = t40731 * t10782;
    let t40735 = t64 * t33127 * t159;
    let t40737 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t40735 * t222;
    let t40744 = t10777 * t10779 * t40578 * t2749;
    let t40746 = F::cast_from(0.10289764348336736874e0_f64) * t2745 * t40673 * t125 * t10627 * t837 + F::cast_from(0.91464571985215438873e-3_f64) * t40679 - F::cast_from(0.16262400898971305032e-1_f64) * t40681 - F::cast_from(0.6098400337114239387e-2_f64) * t40686 + F::cast_from(0.45178982497454656791e-6_f64) * t40691 + F::cast_from(0.17149607247227894789e-3_f64) * t40696 - F::cast_from(0.17149607247227894789e-3_f64) * t40700 + F::cast_from(0.28582678745379824648e-4_f64) * t40705 - F::cast_from(0.13605355082800796532e0_f64) * t40707 - F::cast_from(0.20553867802866510526e-1_f64) * t40711 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t855 * t828 * t39476 - F::cast_from(0.2032800112371413129e-3_f64) * t40719 - F::cast_from(0.73180804045370872643e-3_f64) * t40722 - F::cast_from(0.30492001685571196936e-2_f64) * t40728 - F::cast_from(0.65049603595885220128e-2_f64) * t40732 + t40737 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10900 * t800 * t2731 * t2430 + F::cast_from(0.60984003371142393869e-3_f64) * t40744;
    (t40735, t40746)
}
