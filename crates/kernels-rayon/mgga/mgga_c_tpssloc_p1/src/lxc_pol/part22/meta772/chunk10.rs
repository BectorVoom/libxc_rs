//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2642/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2642(t22337: f64, t225: f64, t22328: f64, t11606: f64, t1235: f64, t1238: f64, t1252: f64, t14980: f64, t1720: f64, t1760: f64, t1761: f64, t19120: f64, t19208: f64, t19214: f64, t19220: f64, t19226: f64, t19232: f64, t19249: f64, t22113: f64, t22394: f64, t3487: f64, t3598: f64, t498: f64, t5055: f64, t5060: f64, t5088: f64, t5089: f64, t6243: f64, t6268: f64, t65208: f64) -> f64 {
    let t73891 = t22337 * t225;
    let t73900 = t22328 * t225;
    let t73919 = -18.0_f64 * t11606 * t1238 * t5088 * t6243 + 6.0_f64 * t1238 * t1760 * t19208 * t3598 + t1235 * t22113 * t498 + 3.0_f64 * t1720 * t19120 * t498 - 3.0_f64 * t1252 * t73891 - t1252 * t73900 - 3.0_f64 * t14980 * t6268 - 3.0_f64 * t1761 * t65208 + 12.0_f64 * t19214 * t5055 + 6.0_f64 * t19220 * t5055 - 18.0_f64 * t19226 * t5055 + 6.0_f64 * t19232 * t5060 - 3.0_f64 * t19249 * t5089 - t22394 * t3487;
    t73919
}
