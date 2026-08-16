//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1144/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1144(t45: f64, t57: f64, t18272: f64, t18277: f64, t18281: f64, t4186: f64, t4377: f64, t606: f64, t78: f64, t10457: f64, t5819: f64, t2382: f64, t5825: f64, t4384: f64, t81: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t18285 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t18272 * t606 + 8.0_f64 / 9.0_f64 * t4377 * t4186 + 4.0_f64 / 9.0_f64 * t18277 * t606 + 4.0_f64 / 3.0_f64 * t78 * t18281);
    let t18286 = t10457 * t5819;
    let t18291 = t2382 * t5825;
    let t18297 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t18286 * t606 + 8.0_f64 / 9.0_f64 * t4384 * t4186 + 4.0_f64 / 9.0_f64 * t18291 * t606 - 4.0_f64 / 3.0_f64 * t81 * t18281);
    (t18285, t18297)
}
