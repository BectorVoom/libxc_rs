//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 941/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk941(t12043: f64, t28371: f64, t28375: f64, t28383: f64, t28391: f64, t28410: f64, t28412: f64, t28415: f64, t28417: f64, t28420: f64, t28423: f64, t28426: f64, t28431: f64, t28435: f64) -> f64 {
    let t29667 = -0.104195e0_f64 * t28410 - 0.52945875e1_f64 * t28412 - t12043 - 0.157790625e0_f64 * t28415 + 0.264729375e1_f64 * t28417 - 0.46308888888888888889e-1_f64 * t28420 - 0.104195e0_f64 * t28423 - 0.62517e0_f64 * t28426 + 0.20659e1_f64 * t28375 - 0.309885e1_f64 * t28383 + 0.20839e0_f64 * t28431 - 0.57386111111111111112e0_f64 * t28371 - 0.516475e0_f64 * t28391 + 0.6311625e0_f64 * t28435;
    t29667
}
