//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2021/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2021(t91143: f64, t91149: f64, t91167: f64, t91179: f64, t93651: f64, t93652: f64, t93653: f64, t93657: f64, t97273: f64, t97277: f64, t97281: f64, t97283: f64, t97287: f64, t97291: f64, t97295: f64, t97299: f64, t97303: f64, t97307: f64) -> f64 {
    let t102679 = -0.80745512188280781708e-3_f64 * t91143 + 0.48447307312968469024e-2_f64 * t97273 + 0.48447307312968469024e-2_f64 * t97277 - 0.48447307312968469024e-2_f64 * t97281 - 119.0_f64 / 432.0_f64 * t91149 + t93651 - t93652 + t93653 - 35.0_f64 / 288.0_f64 * t97283 - 0.16956557559538964158e-1_f64 * t97287 + 0.24223653656484234512e-2_f64 * t97291 + 0.24223653656484234512e-2_f64 * t97295 + 0.24223653656484234512e-2_f64 * t97299 - 0.80745512188280781706e-3_f64 * t97303 - 0.40372756094140390853e-3_f64 * t97307 - 0.45217486825437237755e-1_f64 * t91167 - t93657 - 0.23739180583354549821e0_f64 * t91179;
    t102679
}
