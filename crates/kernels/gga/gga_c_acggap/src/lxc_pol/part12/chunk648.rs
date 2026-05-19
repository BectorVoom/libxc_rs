//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 648/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk648<F: Float>(t1533: F, t301: F, t1165: F, t1532: F, t1531: F, t3462: F, t3666: F, t397: F, t418: F, t5014: F, t5017: F, t5082: F, t5086: F, t5090: F, t5092: F, t5096: F, t5102: F, t5104: F, t5108: F, t5113: F, t5118: F, t5126: F, t5131: F, t5135: F, t5138: F, t5143: F, t5149: F) -> (F, F, F) {
    let t5150 = t1533 * t301;
    let t5152 = t1165 * t1532 * t5150;
    let t5155 = -F::cast_from(0.51448821741683684368e-2_f64) * t418 * t5014 + F::cast_from(0.80031500487063509015e-2_f64) * t5017 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t5082 - t5086 - F::cast_from(0.42874018118069736972e-3_f64) * t5090 - F::cast_from(0.21437009059034868486e-3_f64) * t5092 + F::cast_from(0.25724410870841842183e-2_f64) * t418 * t5096 - F::cast_from(0.85748036236139473944e-3_f64) * t5102 - F::cast_from(0.40015750243531754508e-2_f64) * t5104 + F::cast_from(0.85748036236139473945e-2_f64) * t418 * t5108 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t5113 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t5118 + F::cast_from(0.21437009059034868486e-3_f64) * t3666 + t5126 - t5131 + t5135 + F::cast_from(0.17149607247227894789e-2_f64) * t1531 * t5138 - F::cast_from(0.17149607247227894789e-2_f64) * t1531 * t5143 - t5149 - F::cast_from(0.34299214494455789578e-2_f64) * t3462 * t5152;
    (t5150, t5152, t5155)
}
