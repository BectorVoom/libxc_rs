//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2717/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717(t45: f64, t39858: f64, t14386: f64, t2414: f64, t39860: f64, t10326: f64, t10356: f64, t10446: f64, t11231: f64, t13312: f64, t14401: f64, t14404: f64, t1469: f64, t2251: f64, t2258: f64, t2375: f64, t39825: f64, t4186: f64, t4377: f64, t49889: f64, t606: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t49992 = 12.0_f64 * t39858;
    let t49994 = 12.0_f64 * t14386 * t2414;
    let t49995 = 0.17090684152272775383e-2_f64 * t39860;
    let t50014 = piecewise3(t151, 0.0_f64, 40.0_f64 / 81.0_f64 * t39825 * t1469 * t10356 - 8.0_f64 / 9.0_f64 * t10446 * t4186 * t2251 - 8.0_f64 / 9.0_f64 * t14401 * t11231 + 4.0_f64 / 3.0_f64 * t2375 * t13312 * t606 + 4.0_f64 / 3.0_f64 * t14404 * t2258 + 4.0_f64 / 9.0_f64 * t4377 * t10326 + 4.0_f64 / 3.0_f64 * t78 * t49889);
    (t49992, t49994, t49995, t50014)
}
