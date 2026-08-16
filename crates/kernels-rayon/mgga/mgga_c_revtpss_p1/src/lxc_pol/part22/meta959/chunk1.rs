//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3218/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3218(t57: f64, t10457: f64, t13312: f64, t13396: f64, t14413: f64, t18281: f64, t18286: f64, t18291: f64, t2251: f64, t2258: f64, t2382: f64, t39840: f64, t4384: f64, t5819: f64, t5825: f64, t606: f64, t60717: f64, t60754: f64, t81: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t61085 = piecewise3(t155, 0.0_f64, 40.0_f64 / 81.0_f64 * t39840 * t5819 * t2251 + 32.0_f64 / 27.0_f64 * t14413 * t13396 + 8.0_f64 / 27.0_f64 * t18286 * t2258 + 8.0_f64 / 9.0_f64 * t2382 * t60717 + 8.0_f64 / 9.0_f64 * t4384 * t13312 + 8.0_f64 / 27.0_f64 * t10457 * t5825 * t2251 + 8.0_f64 / 9.0_f64 * t2382 * t18281 * t606 + 4.0_f64 / 9.0_f64 * t18291 * t2258 - 4.0_f64 / 3.0_f64 * t81 * t60754);
    t61085
}
