//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 608/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk608<F: Float>(t1181: F, t1532: F, t5122: F, t1163: F, t1539: F, t372: F, t1165: F, t1552: F, t4210: F, t1533: F, t360: F, t4241: F, t3456: F, t301: F, t1531: F, t3462: F, t3666: F, t397: F, t418: F, t5014: F, t5017: F, t5082: F, t5086: F, t5090: F, t5092: F, t5096: F, t5102: F, t5104: F, t5108: F, t5113: F, t5118: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5124 = t1181 * t1532 * t5122;
    let t5126 = 0.85748036236139473944e-3 * t1163 * t5124;
    let t5127 = t1539 * t372;
    let t5129 = t1165 * t1552 * t5127;
    let t5131 = 0.85748036236139473944e-3 * t1163 * t5129;
    let t5133 = t1165 * t1532 * t4210;
    let t5135 = 0.42874018118069736972e-3 * t1163 * t5133;
    let t5136 = t1533 * t360;
    let t5138 = t1181 * t1532 * t5136;
    let t5141 = t1533 * t372;
    let t5143 = t1165 * t1552 * t5141;
    let t5147 = t1165 * t1532 * t4241;
    let t5149 = 0.85748036236139473944e-3 * t3456 * t5147;
    let t5150 = t1533 * t301;
    let t5152 = t1165 * t1532 * t5150;
    let t5155 = -0.51448821741683684368e-2 * t418 * t5014 + 0.80031500487063509015e-2 * t5017 - 0.21437009059034868486e-3 * t397 * t5082 - t5086 - 0.42874018118069736972e-3 * t5090 - 0.21437009059034868486e-3 * t5092 + 0.25724410870841842183e-2 * t418 * t5096 - 0.85748036236139473944e-3 * t5102 - 0.40015750243531754508e-2 * t5104 + 0.85748036236139473945e-2 * t418 * t5108 - 0.34299214494455789578e-2 * t418 * t5113 - 0.17149607247227894789e-2 * t418 * t5118 + 0.21437009059034868486e-3 * t3666 + t5126 - t5131 + t5135 + 0.17149607247227894789e-2 * t1531 * t5138 - 0.17149607247227894789e-2 * t1531 * t5143 - t5149 - 0.34299214494455789578e-2 * t3462 * t5152;
    (t5124, t5127, t5129, t5133, t5136, t5138, t5141, t5143, t5147, t5150, t5152, t5155)
}
