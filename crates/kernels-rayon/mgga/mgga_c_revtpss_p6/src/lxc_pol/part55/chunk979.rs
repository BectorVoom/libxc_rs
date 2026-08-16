//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 979/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk979(t28902: f64, t689: f64, t225: f64, t28888: f64, t27899: f64, t7515: f64, t2097: f64, t3999: f64) -> (f64, f64, f64, f64) {
    let t28903 = t689 * t28902;
    let t28905 = t28888 * t225;
    let t28909 = t27899 * t7515;
    let t28911 = t3999 * t2097;
    (t28903, t28905, t28909, t28911)
}
