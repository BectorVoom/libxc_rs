//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1123/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1123(t37584: f64, t37588: f64, t37600: f64, t39452: f64, t39455: f64, t39459: f64, t39460: f64, t39462: f64, t39464: f64, t39467: f64, t39470: f64, t39476: f64) -> f64 {
    let t39478 = 0.17336443480108537126e0_f64 * t39452 - 0.5200933044032561138e0_f64 * t39455 - t39459 + 0.86682217400542685632e-1_f64 * t39460 + 0.2600466522016280569e0_f64 * t39462 - 0.59512461497092438715e-1_f64 * t39464 + 0.5200933044032561138e0_f64 * t39467 - 0.14457274399185490173e-3_f64 * t39470 - 0.28565981518604370583e-1_f64 * t37584 - 0.47609969197673950972e-2_f64 * t37588 - t37600 - 0.21831846657716620896e-2_f64 * t39476;
    t39478
}
