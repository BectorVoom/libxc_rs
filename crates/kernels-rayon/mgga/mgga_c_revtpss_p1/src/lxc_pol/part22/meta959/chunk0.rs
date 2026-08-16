//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3217/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217(t45: f64, t39438: f64, t49876: f64, t11064: f64, t6075: f64, t37: f64, t5940: f64, t2612: f64, t10446: f64, t13312: f64, t13396: f64, t14401: f64, t18272: f64, t18277: f64, t18281: f64, t2251: f64, t2258: f64, t2375: f64, t39825: f64, t4377: f64, t5819: f64, t5825: f64, t606: f64, t60717: f64, t60754: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t61031 = 0.32530743900905219526e-1_f64 * t39438;
    let t61032 = 48.0_f64 * t49876;
    let t61033 = t6075 * t11064;
    let t61037 = t37 * t5940;
    let t61039 = 12.0_f64 * t61037 * t2612;
    let t61062 = piecewise3(t151, 0.0_f64, 40.0_f64 / 81.0_f64 * t39825 * t5819 * t2251 - 32.0_f64 / 27.0_f64 * t14401 * t13396 - 8.0_f64 / 27.0_f64 * t18272 * t2258 + 8.0_f64 / 9.0_f64 * t2375 * t60717 + 8.0_f64 / 9.0_f64 * t4377 * t13312 - 8.0_f64 / 27.0_f64 * t10446 * t5825 * t2251 + 8.0_f64 / 9.0_f64 * t2375 * t18281 * t606 + 4.0_f64 / 9.0_f64 * t18277 * t2258 + 4.0_f64 / 3.0_f64 * t78 * t60754);
    (t61031, t61032, t61033, t61039, t61062)
}
