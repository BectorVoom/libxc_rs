//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1696/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1696(t21720: f64, t21808: f64, t10301: f64, t10309: f64, t13269: f64, t13272: f64, t1497: f64, t21661: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t4241: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t91: f64) -> (f64, f64) {
    let t21809 = t21720 + t21808;
    let t21812 = 20.0_f64 * t10301 * t5816 - 120.0_f64 * t10309 * t21674 - 8.0_f64 * t13269 * t1497 + 40.0_f64 * t13272 * t4178 + t21661 * t91 - 4.0_f64 * t21663 * t644 + 40.0_f64 * t21677 * t2247 + 20.0_f64 * t21682 * t2247 - 4.0_f64 * t21809 * t603 - 4.0_f64 * t2242 * t5872 - 8.0_f64 * t4173 * t4241;
    (t21809, t21812)
}
