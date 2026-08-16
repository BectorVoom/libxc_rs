//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1977/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1977(t20545: f64, t20602: f64, t20640: f64, t20682: f64, t300: f64, t20568: f64, t20261: f64, t20263: f64, t20386: f64, t20388: f64, t20390: f64, t20393: f64, t20396: f64, t20399: f64, t20402: f64, t20404: f64, t20450: f64, t20452: f64, t20454: f64, t20471: f64, t20475: f64, t20477: f64, t5023: f64, t5501: f64, t5505: f64) -> (f64, f64, f64) {
    let t20685 = t300 * (t20545 + t20602 + t20640 + t20682);
    let t20690 = 0.19751673498613801407e-1_f64 * t300 * t20568;
    let t20691 = -2.0_f64 * t5023 * t5501 * t5505 - t20261 - t20263 - t20386 - t20388 - t20390 - t20393 + t20396 - t20399 - t20402 - t20404 + t20450 + t20452 + t20454 - t20471 + t20475 + t20477 + t20685 + t20690;
    (t20685, t20690, t20691)
}
