//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1178/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1178<F: Float>(t1165: F, t2068: F, t39596: F, t7351: F, t31350: F, t5737: F, t7337: F, t8480: F, t8902: F, t35836: F, t35838: F, t35845: F, t40196: F, t40200: F, t40204: F, t40208: F, t40212: F, t40218: F, t40220: F, t40222: F, t40226: F, t40230: F, t40234: F, t40237: F) -> F {
    let t40241 = t2068 * t1165 * t7351 * t39596;
    let t40243 = t31350 * t5737;
    let t40246 = t7337 * t8480 * t8902;
    let t40248 = F::cast_from(0.31448092289604152068e-2_f64) * t40196 + F::cast_from(0.31448092289604152068e-2_f64) * t40200 + F::cast_from(0.20965394859736101379e-2_f64) * t40204 - F::cast_from(0.10718504529517434243e-3_f64) * t40208 - F::cast_from(0.32155513588552302729e-2_f64) * t40212 + F::cast_from(0.94344276868812456204e-3_f64) * t40218 - F::cast_from(0.15724046144802076034e-2_f64) * t40220 - t35836 + t35838 - F::cast_from(0.64311027177104605458e-3_f64) * t40222 + F::cast_from(0.31448092289604152068e-3_f64) * t40226 + F::cast_from(0.31448092289604152068e-3_f64) * t40230 + F::cast_from(0.15724046144802076034e-3_f64) * t40234 + t35845 + F::cast_from(0.21437009059034868486e-3_f64) * t40237 - F::cast_from(0.47172138434406228102e-3_f64) * t40241 - F::cast_from(0.17149607247227894789e-1_f64) * t40243 + F::cast_from(0.10718504529517434243e-2_f64) * t40246;
    t40248
}
