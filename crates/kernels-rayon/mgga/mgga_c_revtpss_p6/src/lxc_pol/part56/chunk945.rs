//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 945/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk945(t32243: f64, t32295: f64, t532: f64, t1450: f64, t2014: f64, t7003: f64, t8634: f64, t32171: f64, t508: f64, t1310: f64, t8454: f64, t1459: f64, t8611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32296 = t32243 + t32295;
    let t32297 = t532 * t32296;
    let t32298 = t32297 * t1450;
    let t32299 = t2014 * t32298;
    let t32320 = 4.0_f64 * t8634 * t7003;
    let t32338 = 2.0_f64 * t32171 * t508;
    let t32340 = 2.0_f64 * t8454 * t1310;
    let t32365 = 6.0_f64 * t1459 * t8611;
    (t32296, t32297, t32298, t32299, t32320, t32338, t32340, t32365)
}
