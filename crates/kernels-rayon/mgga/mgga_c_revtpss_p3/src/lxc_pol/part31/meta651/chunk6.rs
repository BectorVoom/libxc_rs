//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2158/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2158(t27479: f64, t4845: f64, t100324: f64, t100359: f64, t100363: f64, t100365: f64, t100370: f64, t100398: f64, t1665: f64, t19645: f64, t19917: f64, t25517: f64, t25539: f64, t6289: f64, t6339: f64, t7111: f64, t93731: f64) -> f64 {
    let t107188 = t27479 * t4845;
    let t107197 = 0.28582678745379824648e-3_f64 * t25517 * t19645 + t100359 + 0.45732285992607719437e-2_f64 * t100324 * t1665 - 0.57165357490759649296e-3_f64 * t107188 + 0.85748036236139473944e-3_f64 * t93731 * t6339 - t100363 - t100365 / 648.0_f64 + t100370 - t100398 + t7111 * t19917 / 288.0_f64 - t25539 * t6289 / 108.0_f64;
    t107197
}
