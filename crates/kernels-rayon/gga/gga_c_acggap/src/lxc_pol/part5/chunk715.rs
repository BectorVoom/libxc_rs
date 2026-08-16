//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 715/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk715(t1533: f64, t301: f64, t1165: f64, t1532: f64, t1531: f64, t3462: f64, t3666: f64, t397: f64, t418: f64, t5014: f64, t5017: f64, t5082: f64, t5086: f64, t5090: f64, t5092: f64, t5096: f64, t5102: f64, t5104: f64, t5108: f64, t5113: f64, t5118: f64, t5126: f64, t5131: f64, t5135: f64, t5138: f64, t5143: f64, t5149: f64) -> (f64, f64, f64) {
    let t5150 = t1533 * t301;
    let t5152 = t1165 * t1532 * t5150;
    let t5155 = -0.51448821741683684368e-2_f64 * t418 * t5014 + 0.80031500487063509015e-2_f64 * t5017 - 0.21437009059034868486e-3_f64 * t397 * t5082 - t5086 - 0.42874018118069736972e-3_f64 * t5090 - 0.21437009059034868486e-3_f64 * t5092 + 0.25724410870841842183e-2_f64 * t418 * t5096 - 0.85748036236139473944e-3_f64 * t5102 - 0.40015750243531754508e-2_f64 * t5104 + 0.85748036236139473945e-2_f64 * t418 * t5108 - 0.34299214494455789578e-2_f64 * t418 * t5113 - 0.17149607247227894789e-2_f64 * t418 * t5118 + 0.21437009059034868486e-3_f64 * t3666 + t5126 - t5131 + t5135 + 0.17149607247227894789e-2_f64 * t1531 * t5138 - 0.17149607247227894789e-2_f64 * t1531 * t5143 - t5149 - 0.34299214494455789578e-2_f64 * t3462 * t5152;
    (t5150, t5152, t5155)
}
