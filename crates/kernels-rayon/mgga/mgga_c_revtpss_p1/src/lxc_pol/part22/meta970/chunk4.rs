//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3242/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242(t57: f64, t13312: f64, t13396: f64, t1491: f64, t18281: f64, t18379: f64, t18384: f64, t2251: f64, t2258: f64, t4335: f64, t5864: f64, t5866: f64, t606: f64, t60717: f64, t60754: f64, t770: f64, t83: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t61517 = piecewise3(t155, 0.0_f64, -56.0_f64 / 81.0_f64 * t5864 * t2251 - 32.0_f64 / 27.0_f64 * t1491 * t13396 - 8.0_f64 / 27.0_f64 * t18379 * t2258 - 4.0_f64 / 9.0_f64 * t83 * t60717 - 4.0_f64 / 9.0_f64 * t4335 * t13312 - 8.0_f64 / 27.0_f64 * t5866 * t2251 - 4.0_f64 / 9.0_f64 * t83 * t18281 * t606 - 2.0_f64 / 9.0_f64 * t18384 * t2258 - 2.0_f64 / 3.0_f64 * t770 * t60754);
    t61517
}
