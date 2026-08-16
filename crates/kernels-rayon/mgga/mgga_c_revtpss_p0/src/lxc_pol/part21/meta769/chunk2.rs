//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2725/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725(t57: f64, t10326: f64, t10356: f64, t11231: f64, t13312: f64, t14458: f64, t1491: f64, t2251: f64, t2258: f64, t4232: f64, t4235: f64, t4335: f64, t49889: f64, t606: f64, t770: f64, t83: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t50149 = piecewise3(t155, 0.0_f64, -56.0_f64 / 81.0_f64 * t4232 * t10356 - 8.0_f64 / 9.0_f64 * t4235 * t2251 - 8.0_f64 / 9.0_f64 * t1491 * t11231 - 2.0_f64 / 3.0_f64 * t83 * t13312 * t606 - 2.0_f64 / 3.0_f64 * t14458 * t2258 - 2.0_f64 / 9.0_f64 * t4335 * t10326 - 2.0_f64 / 3.0_f64 * t770 * t49889);
    t50149
}
