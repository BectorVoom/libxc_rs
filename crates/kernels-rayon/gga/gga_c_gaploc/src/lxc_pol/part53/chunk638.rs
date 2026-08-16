//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 638/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk638(t11986: f64, t447: f64, t1064: f64, t3701: f64, t448: f64, t3691: f64, t535: f64, t10115: f64, t10118: f64, t10131: f64, t10134: f64, t10137: f64, t10139: f64, t1063: f64, t11978: f64, t11983: f64, t2268: f64, t9072: f64, t9077: f64, t9085: f64) -> (f64, f64) {
    let t11987 = t11986 * t447;
    let t11988 = t1064 * t11987;
    let t11991 = t3701 * t448;
    let t11994 = t535 * t3691;
    let t11997 = -0.85365019907028448797e-1_f64 * t2268 * t11978 + 0.56910013271352299198e-1_f64 * t2268 * t11983 + 0.28455006635676149599e-1_f64 * t1063 * t11988 - 0.28455006635676149599e-1_f64 * t1063 * t11991 + 0.28455006635676149599e-1_f64 * t2268 * t11994 - t9072 + t9077 + t9085 + t10115 + t10118 - t10131 - t10134 - t10137 + t10139;
    (t11987, t11997)
}
