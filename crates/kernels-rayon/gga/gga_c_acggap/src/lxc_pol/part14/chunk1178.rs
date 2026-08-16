//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1178/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1178(t1165: f64, t2068: f64, t39596: f64, t7351: f64, t31350: f64, t5737: f64, t7337: f64, t8480: f64, t8902: f64, t35836: f64, t35838: f64, t35845: f64, t40196: f64, t40200: f64, t40204: f64, t40208: f64, t40212: f64, t40218: f64, t40220: f64, t40222: f64, t40226: f64, t40230: f64, t40234: f64, t40237: f64) -> f64 {
    let t40241 = t2068 * t1165 * t7351 * t39596;
    let t40243 = t31350 * t5737;
    let t40246 = t7337 * t8480 * t8902;
    let t40248 = 0.31448092289604152068e-2_f64 * t40196 + 0.31448092289604152068e-2_f64 * t40200 + 0.20965394859736101379e-2_f64 * t40204 - 0.10718504529517434243e-3_f64 * t40208 - 0.32155513588552302729e-2_f64 * t40212 + 0.94344276868812456204e-3_f64 * t40218 - 0.15724046144802076034e-2_f64 * t40220 - t35836 + t35838 - 0.64311027177104605458e-3_f64 * t40222 + 0.31448092289604152068e-3_f64 * t40226 + 0.31448092289604152068e-3_f64 * t40230 + 0.15724046144802076034e-3_f64 * t40234 + t35845 + 0.21437009059034868486e-3_f64 * t40237 - 0.47172138434406228102e-3_f64 * t40241 - 0.17149607247227894789e-1_f64 * t40243 + 0.10718504529517434243e-2_f64 * t40246;
    t40248
}
