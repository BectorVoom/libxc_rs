//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2937/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937(t43002: f64, t60274: f64, t60277: f64, t60282: f64, t60296: f64, t60308: f64, t60310: f64, t60312: f64, t60315: f64, t60318: f64, t60321: f64, t60324: f64, t60327: f64) -> f64 {
    let t61163 = -t43002 - 2.0_f64 / 27.0_f64 * t60274 - 2.0_f64 / 3.0_f64 * t60277 - t60282 / 3.0_f64 - t60296 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t60308 - 4.0_f64 / 27.0_f64 * t60310 - 8.0_f64 / 81.0_f64 * t60312 - t60315 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t60318 + t60321 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t60324 + 14.0_f64 / 81.0_f64 * t60327;
    t61163
}
