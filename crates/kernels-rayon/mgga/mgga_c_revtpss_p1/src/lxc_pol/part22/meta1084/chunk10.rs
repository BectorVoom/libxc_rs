//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3935/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3935(t116: f64, t117: f64, t1459: f64, t18190: f64, t18208: f64, t1916: f64, t1918: f64, t22556: f64, t22559: f64, t22568: f64, t2327: f64, t2371: f64, t4162: f64, t4165: f64, t4292: f64, t572: f64, t573: f64, t5883: f64, t5920: f64, t60595: f64, t6941: f64, t75494: f64, t75657: f64, t75716: f64, param_d: f64) -> f64 {
    let t75792 = 12.0_f64 * t116 * t572 * t75494 + 3.0_f64 * t117 * t572 * t75657 + 6.0_f64 * t2327 * t572 * t5920 + 6.0_f64 * t2371 * t572 * t5883 + 24.0_f64 * t4292 * t572 * t60595 + t573 * t75716 * param_d + 12.0_f64 * t1459 * t22556 + 24.0_f64 * t1459 * t22559 + 6.0_f64 * t1459 * t22568 + 6.0_f64 * t18190 * t1918 + 24.0_f64 * t18208 * t1916 + 6.0_f64 * t4162 * t6941 + 3.0_f64 * t4165 * t6941;
    t75792
}
